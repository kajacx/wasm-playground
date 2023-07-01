use js_sys::{Function, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn compute() -> String {
    console_error_panic_hook::set_once();
    compute_it().await
}

async fn compute_it() -> String {
    // let window = web_sys::window().unwrap();

    // let import = Reflect::get(&window.into(), &"import".into()).expect("get import");
    // panic!("IMPORT IS: {import:?}");

    let imported = js_sys::eval("import('/out-dir/component.js')").expect("eval import");
    let imported = await_js_value(imported).await;

    let instantiate = Reflect::get(&imported, &"instantiate".into()).expect("get instantiate");
    let instantiate: Function = instantiate.try_into().expect("get as fn");

    let compile_core = js_sys::eval(
        r#"async (url, imports) => {
        let data = await fetch("/out-dir/" + url);
        return WebAssembly.compile(await data.arrayBuffer(), imports);
      }"#,
    )
    .expect("eval compile core");

    let import_object: JsValue = js_sys::Object::new().into();
    let log = js_sys::eval("(val) => console.log('VALUE IS:', val)").expect("eval log");
    Reflect::set(&import_object, &"default".into(), &log).unwrap();

    let instance = instantiate
        .call2(&imported, &compile_core, &import_object)
        .expect("call instantiate");
    let instance = await_js_value(instance).await;

    let run = Reflect::get(&instance, &"run".into()).expect("get run");
    let run: Function = run.try_into().expect("get run as fn");

    run.call0(&instance).expect("should call run");

    // panic!("INSTANCE IS what?: {instance:?}");

    "ffooo".into()
}

async fn await_js_value(value: JsValue) -> JsValue {
    let as_promise = js_sys::Promise::try_from(value).expect("value to promise");
    JsFuture::from(as_promise).await.expect("awaiting promise")
}
