use deno_core::{
    anyhow::Result, serde::de::DeserializeOwned, serde_v8, v8, JsRuntime, RuntimeOptions,
};

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions::default();
    let mut rt = JsRuntime::new(options);
    let code = include_str!("basic.js");
    let ret: String = eval(&mut rt, code).await?;
    println!("{:?}", ret);
    Ok(())
}

async fn eval<T>(rt: &mut JsRuntime, code: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let ret = rt.execute_script("demo", code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = v8::Local::new(scope, result);
    Ok(serde_v8::from_v8(scope, result)?)
}
