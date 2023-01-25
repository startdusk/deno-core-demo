use std::rc::Rc;

use deno_core::{anyhow::Result, resolve_url_or_path, FsModuleLoader, JsRuntime, RuntimeOptions};

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);

    let path = format!("{}/examples/load_module.js", env!("CARGO_MANIFEST_DIR"));
    execute_main_module(&mut rt, path).await?;
    Ok(())
}

#[allow(dead_code)]
async fn execute_main_module(rt: &mut JsRuntime, path: impl AsRef<str>) -> Result<()> {
    let url = resolve_url_or_path(path.as_ref())?;
    let id = rt.load_main_module(&url, None).await?;
    let mut receiver = rt.mod_evaluate(id);

    tokio::select! {
        resolved = &mut receiver => {
            resolved.expect("failed to evaluate module")
        }
        _ = rt.run_event_loop(false) => {
            receiver.await.expect("faild to evaluate module")
        }
    }
}
