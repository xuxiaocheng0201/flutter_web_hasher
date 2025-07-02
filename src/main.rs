use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

fn rename_files(
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

        let stem = path.file_stem().unwrap_or(OsStr::new(""));
        let ext = path.extension().unwrap_or(OsStr::new(""));

        let buffer = std::fs::read(path)?;
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(&buffer);
        let hash = hex::encode(hasher.finalize())[..8].to_string();

        let mut new_name = stem.to_os_string();
        new_name.push(".");
        new_name.push(hash);
        if !ext.is_empty() {
            new_name.push(".");
            new_name.push(ext);
        }

        let mut new_path = path.to_path_buf();
        new_path.set_file_name(&new_name);
        std::fs::rename(path, &new_path)?;

        map.insert(path.to_path_buf(), new_path);
    }
    Ok(map)
}

fn update_references(
    directory: impl AsRef<Path>,
    replace_base: &[&Path],
    manifest: &HashMap<PathBuf, PathBuf>,
    skip_list: &[&Path],
    replace_wrapper: impl Fn(&str) -> Vec<String> + Copy,
) -> Result<()> {
    let directory = directory.as_ref();
    let replace_base = if replace_base.is_empty() {
        &[Path::new("")]
    } else { replace_base }; // empty base should be the first one.

    let mut patterns = Vec::with_capacity(manifest.len() * replace_base.len());
    let mut replacements = Vec::with_capacity(manifest.len() * replace_base.len());
    for replace_base in replace_base {
        let replace_base = directory.join(replace_base);
        for (old_path, new_path) in manifest {
            let Ok(old_relative) = old_path.strip_prefix(&replace_base) else { continue };
            let old_relative = old_relative.to_str().ok_or_else(|| anyhow!("Old path is not a valid utf-8. {}", old_path.display()))?;
            let Ok(new_relative) = new_path.strip_prefix(&replace_base) else { continue };
            let new_relative = new_relative.to_str().ok_or_else(|| anyhow!("New path is not a valid utf-8. {}", new_path.display()))?;

            let olds = replace_wrapper(old_relative);
            let news = replace_wrapper(new_relative);
            debug_assert_eq!(olds.len(), news.len());
            for old in olds { patterns.push(old); }
            for new in news { replacements.push(new); }
        }
    }

    for entry in walkdir::WalkDir::new(directory) {
        let entry = entry?;
        if !entry.file_type().is_file() { continue; }
        let path = entry.path();
        if skip_list.contains(&path.strip_prefix(directory)?) { continue; }

        if let Ok(content) = std::fs::read_to_string(path) {
            let ac = aho_corasick::AhoCorasick::new(&patterns).expect("failed to create AhoCorasick");
            let new_content = ac.replace_all(&content, &replacements);
            if content != new_content {
                std::fs::write(path, new_content)?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let manifest = rename_files(
        "build/web",
        &[
            Path::new("index.html"),
            Path::new("404.html"),
        ],
    )?;
    update_references(
        "build/web",
        &[Path::new(""), Path::new("assets")],
        &manifest,
        &[],
        |context| if context.ends_with("flutter_service_worker.js") {
            vec![context.to_string()]
        } else {
            vec![format!("\"{context}\"")]
        },
    )?;
    Ok(())
}
