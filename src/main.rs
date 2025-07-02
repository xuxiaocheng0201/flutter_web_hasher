use std::path::Path;

fn main() -> anyhow::Result<()> {
    let replace_middle = |index| format!("@@@FWH#{index}#FWH@@@");
    
    let manifest = flutter_web_hasher::rename_files(
        "build/web",
        &[Path::new("index.html")]
    )?;
    flutter_web_hasher::update_references(
        "build/web",
        &[Path::new(""), Path::new("assets")],
        &manifest,
        &[],
        replace_middle
    )?;
    Ok(())
}
