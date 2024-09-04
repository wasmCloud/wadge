use std::env;
use std::path::Path;

use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    let crate_dir =
        env::var("CARGO_MANIFEST_DIR").context("failed to lookup `CARGO_MANIFEST_DIR`")?;
    let bindings = cbindgen::generate_with_config(
        &crate_dir,
        cbindgen::Config {
            language: cbindgen::Language::C,
            ..Default::default()
        },
    )
    .context("failed to generate bindings")?;
    bindings.write_to_file(
        Path::new(&crate_dir)
            .join("..")
            .join("..")
            .join("include")
            .join("west.h"),
    );
    Ok(())
}
