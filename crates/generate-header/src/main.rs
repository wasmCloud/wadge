use std::env;
use std::path::PathBuf;

use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let crate_dir =
        env::var("CARGO_MANIFEST_DIR").context("failed to lookup `CARGO_MANIFEST_DIR`")?;
    let crate_dir = PathBuf::from(crate_dir);
    let crates = crate_dir
        .parent()
        .context("failed to lookup crate parent directory")?;
    let root = crates
        .parent()
        .context("failed to lookup workspace root directory")?;
    let bindings = cbindgen::generate_with_config(
        crates.join("wadge-sys"),
        cbindgen::Config {
            language: cbindgen::Language::C,
            ..Default::default()
        },
    )
    .context("failed to generate bindings")?;
    bindings.write_to_file(root.join("include").join("wadge.h"));
    Ok(())
}
