use std::fmt::Debug;

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Instance {
    run: Function,
    move_point: Function,
    _drop_handle: Box<dyn Debug>,
}

impl Instance {
    pub fn run(&self) {
        self.run.call0(&JsValue::UNDEFINED).expect("Call run");
    }

    pub fn move_point(&self, point: Point) -> Point {
        let point = point_to_js_value(&point);
        let point = self
            .move_point
            .call1(&JsValue::UNDEFINED, &point)
            .expect("Call move point");
        point_from_js_value(&point)
    }
}

pub async fn instantiate(import_point: impl Fn(Point) -> Point + 'static) -> Instance {
    let imported = js_sys::eval("import('/out-dir/component.js')").expect("eval import");
    let imported = await_js_value(imported).await;

    let instantiate: Function = Reflect::get(&imported, &"instantiate".into())
        .expect("get instantiate")
        .into();

    let compile_core = js_sys::eval(
        r#"async (url, imports) => {
        let data = await fetch("/out-dir/" + url);
        return WebAssembly.compile(await data.arrayBuffer(), imports);
      }"#,
    )
    .expect("eval compile core");

    let print = js_sys::eval("console.log").unwrap();

    let import_point = Closure::<dyn Fn(JsValue) -> JsValue>::new(move |point: JsValue| {
        let point = point_from_js_value(&point);
        let point = import_point(point);
        point_to_js_value(&point)
    });

    let import_object: JsValue = js_sys::Object::new().into();
    Reflect::set(&import_object, &"print".into(), &wrap_import_fn(&print)).unwrap();
    Reflect::set(
        &import_object,
        &"import-point".into(),
        &wrap_import_fn(&import_point.as_ref().into()),
    )
    .unwrap();

    let instance = instantiate
        .call2(&imported, &compile_core, &import_object)
        .expect("call instantiate");
    let instance = await_js_value(instance).await;

    let run: Function = Reflect::get(&instance, &"run".into())
        .expect("get run")
        .into();
    let move_point: Function = Reflect::get(&instance, &"movePoint".into())
        .expect("get move point")
        .into();

    Instance {
        run,
        move_point,
        _drop_handle: Box::new(import_point),
    }
}

fn point_from_js_value(value: &JsValue) -> Point {
    let x = Reflect::get(&value, &"y".into()).unwrap().as_f64().unwrap() as i32;
    let y = Reflect::get(&value, &"y".into()).unwrap().as_f64().unwrap() as i32;
    Point { x, y }
}

fn point_to_js_value(point: &Point) -> JsValue {
    let result: JsValue = Object::new().into();
    Reflect::set(&result, &"x".into(), &point.x.into()).unwrap();
    Reflect::set(&result, &"y".into(), &point.y.into()).unwrap();
    result
}

fn wrap_import_fn(import_fn: &JsValue) -> JsValue {
    let result = Object::new().into();
    Reflect::set(&result, &"default".into(), import_fn).unwrap();
    result
}

async fn await_js_value(value: JsValue) -> JsValue {
    let as_promise = js_sys::Promise::try_from(value).expect("value to promise");
    JsFuture::from(as_promise).await.expect("awaiting promise")
}
