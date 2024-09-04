use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

use anyhow::{ensure, Context as _};
use wasi_preview1_component_adapter_provider::{
    WASI_SNAPSHOT_PREVIEW1_ADAPTER_NAME, WASI_SNAPSHOT_PREVIEW1_REACTOR_ADAPTER,
};

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=crates/passthrough");

    let out_dir = env::var("OUT_DIR")
        .map(PathBuf::from)
        .context("failed to lookup `OUT_DIR`")?;
    let status = Command::new(env::var("CARGO").unwrap())
        .args([
            "build",
            "-p",
            "west-passthrough",
            "--release",
            "--target",
            "wasm32-wasip1",
            "--target-dir",
        ])
        .arg(&out_dir)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .context("failed to invoke `cargo`")?;
    ensure!(status.success(), "`cargo` invocation failed");
    let path = out_dir
        .join("wasm32-wasip1")
        .join("release")
        .join("west_passthrough.wasm");
    let module = fs::read(&path).with_context(|| format!("failed to read `{}`", path.display()))?;
    let component = wit_component::ComponentEncoder::default()
        .validate(true)
        .module(&module)
        .context("failed to set core component module")?
        .adapter(
            WASI_SNAPSHOT_PREVIEW1_ADAPTER_NAME,
            WASI_SNAPSHOT_PREVIEW1_REACTOR_ADAPTER,
        )
        .context("failed to add WASI adapter")?
        .encode()
        .with_context(|| format!("failed to encode `{}`", path.display()))?;

    let path = out_dir.join("west_passthrough.wasm");
    fs::write(&path, component).with_context(|| format!("failed to write `{}`", path.display()))?;

    Ok(())
}
