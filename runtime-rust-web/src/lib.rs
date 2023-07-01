use std::{
    cell::Ref,
    sync::{Arc, Mutex},
};

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn compute() -> String {
    console_error_panic_hook::set_once();
    compute_it().await
}

async fn compute_it() -> String {
    let text = Arc::new(Mutex::new(String::new()));

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

    let text_clone = text.clone();
    let print = Closure::<dyn Fn(String)>::new(move |value: String| {
        text_clone.try_lock().unwrap().push_str(&value);
    });

    let point_import = Closure::<dyn Fn(Point) -> Point>::new(|mut point: Point| {
        point.x += 100;
        point.y += 1000;
        point
    });

    let import_object: JsValue = js_sys::Object::new().into();
    // let log = js_sys::eval("(val) => console.log('VALUE IS:', val)").expect("eval log");
    Reflect::set(&import_object, &"default".into(), &print.as_ref().into()).unwrap();
    Reflect::set(
        &import_object,
        &"default".into(),
        &point_import.as_ref().into(),
    )
    .unwrap();

    let instance = instantiate
        .call2(&imported, &compile_core, &import_object)
        .expect("call instantiate");
    let instance = await_js_value(instance).await;

    let run = Reflect::get(&instance, &"run".into()).expect("get run");
    let run: Function = run.try_into().expect("get run as fn");

    run.call0(&instance).expect("should call run");

    let console = js_sys::eval("console").unwrap();
    let log = Reflect::get(&console, &"log".into()).expect("get log");
    let log: Function = log.try_into().expect("get log as fn");

    let point = Point { x: 10, y: 10 };
    let point: JsValue = point.into();
    // let point: JsValue = Object::new().into();
    // Reflect::set(&point, &"x".into(), &20.into()).expect("set x");
    // Reflect::set(&point, &"y".into(), &20.into()).expect("set y");
    log.call2(&console, &"HELLO POINT".into(), &point).unwrap();

    let move_point = Reflect::get(&instance, &"movePoint".into()).expect("get move point");
    // let mm = move_point.clone();
    let move_point: Function = move_point.try_into().expect("get move point as fn");

    log.call2(&console, &"HELLO CONSOLE".into(), &instance)
        .unwrap();

    //web_sys::window().unwrap().alert();
    // panic!("WHAT IS IT then? {mm:?}, {move_point:?}");

    let point = move_point
        .call1(&instance, &point)
        .expect("pls call move point");
    // let point: Point = point.try_into().expect("get point into");
    let point = Point {
        x: Reflect::get(&point, &"x".into())
            .expect("get x")
            .as_f64()
            .expect("unwrap x") as _,
        y: 0,
    };

    drop(print);

    let text = text.try_lock().unwrap();
    let text = text.clone();

    format!("{text} AND {point:?}")
}

async fn await_js_value(value: JsValue) -> JsValue {
    let as_promise = js_sys::Promise::try_from(value).expect("value to promise");
    JsFuture::from(as_promise).await.expect("awaiting promise")
}

#[wasm_bindgen]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
