use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

fn get_file_new_name(file: impl AsRef<Path>) -> Result<OsString> {
    let path = file.as_ref();
    let stem = path.file_stem().unwrap_or(OsStr::new(""));
    let ext = path.extension().unwrap_or(OsStr::new(""));

    let buffer = std::fs::read(path)?;
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(&buffer);
    let hash1 = hex::encode(hasher.finalize())[..8].to_string();
    let hash = hash1;

    let mut new_name = stem.to_os_string();
    new_name.push(".");
    new_name.push(hash);
    if !ext.is_empty() {
        new_name.push(".");
        new_name.push(ext);
    }

    Ok(new_name)
}

pub fn rename_files(
    directory: impl AsRef<Path>,
    skip_list: &[&Path]
) -> Result<HashMap<PathBuf, PathBuf>> {
    let directory = directory.as_ref();
    let mut map = HashMap::new();
    for entry in walkdir::WalkDir::new(directory) {
        let entry = entry?;
        if !entry.file_type().is_file() || entry.path_is_symlink() { continue; }
        let path = entry.path();
        if skip_list.contains(&path.strip_prefix(directory)?) { continue; }

        let new_name = get_file_new_name(path)?;
        let mut new_path = path.to_path_buf();
        new_path.set_file_name(&new_name);
        std::fs::rename(path, &new_path)?;

        map.insert(path.to_path_buf(), new_path);
    }
    Ok(map)
}

fn replace_content(
    content: &str,
    replacer: &[(&str, &str)],
    replace_middle: impl Fn(usize) -> String,
    replace_wrapper: impl Fn(&str) -> Vec<String>,
) -> String {
    let mut content = content.to_string();
    let mut helper = Vec::with_capacity(replacer.len());
    let mut index = 0;
    for &(old, new) in replacer {
        let olds = replace_wrapper(old);
        let news = replace_wrapper(new);
        debug_assert_eq!(olds.len(), news.len());
        for (old, new) in olds.into_iter().zip(news.into_iter()) {
            let middle = replace_middle(index);
            helper.push((old, middle, new));
            index += 1;
        }
    }
    for (old, middle, _new) in &helper {
        content = content.replace(old, middle);
    }
    for (_old, middle, new) in &helper {
        content = content.replace(middle, new);
    }
    content
}

pub fn update_references(
    directory: impl AsRef<Path>,
    replace_base: &[&Path],
    manifest: &HashMap<PathBuf, PathBuf>,
    skip_list: &[&Path],
    replace_middle: impl Fn(usize) -> String + Copy,
    replace_wrapper: impl Fn(&str) -> Vec<String> + Copy,
) -> Result<()> {
    let directory = directory.as_ref();
    let replace_base = if replace_base.is_empty() {
        &[Path::new("")]
    } else { replace_base }; // empty base should be the first one.

    let mut converter = Vec::with_capacity(manifest.len());
    for replace_base in replace_base {
        let replace_base = directory.join(replace_base);
        for (old_path, new_path) in manifest {
            let Ok(old_relative) = old_path.strip_prefix(&replace_base) else { continue };
            let old_relative = old_relative.to_str().ok_or_else(|| anyhow!("Old path is not a valid utf-8. {}", old_path.display()))?;
            let Ok(new_relative) = new_path.strip_prefix(&replace_base) else { continue };
            let new_relative = new_relative.to_str().ok_or_else(|| anyhow!("New path is not a valid utf-8. {}", new_path.display()))?;
            converter.push((old_relative, new_relative));
        }
    }

    for entry in walkdir::WalkDir::new(directory) {
        let entry = entry?;
        if !entry.file_type().is_file() { continue; }
        let path = entry.path();
        if skip_list.contains(&path.strip_prefix(directory)?) { continue; }

        if let Ok(content) = std::fs::read_to_string(path) {
            let new_content = replace_content(&content, &converter, replace_middle, replace_wrapper);
            if content != new_content {
                std::fs::write(path, new_content)?;
            }
        }
    }
    Ok(())
}
