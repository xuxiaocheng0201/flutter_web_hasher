use std::path::Path;

fn main() -> anyhow::Result<()> {
    let manifest = flutter_web_hasher::rename_files(
        "build/web",
        &[
            Path::new("index.html"),
            Path::new("404.html"),
        ],
    )?;
    flutter_web_hasher::update_references(
        "build/web",
        &[Path::new(""), Path::new("assets")],
        &manifest,
        &[],
        |index| format!("@@@FWH#{index}#FWH@@@"),
        |context| if context.ends_with("flutter_service_worker.js") {
            vec![context.to_string()]
        } else {
            vec![format!("\"{context}\"")]
        },
    )?;
    Ok(())
}
