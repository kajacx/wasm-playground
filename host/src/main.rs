use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use wasm_runtime_layer::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/bevy_plugin.wasm");

fn main() {
    let (r, g, b) = get_color();
    let bevy_color = Color::rgb(r, g, b);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(ClearColor(bevy_color))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn get_color() -> (f32, f32, f32) {
    let engine = Engine::new(get_engine());
    let mut store = Store::new(&engine, ());

    let module = Module::new(&store.engine(), PLUGIN_BYTES).expect("compile module");

    let instance =
        Instance::new(&mut store, &module, &Imports::default()).expect("create instance");

    let mut result = [Value::I32(0)];

    instance
        .get_export(&store, "get_red")
        .expect("get red export")
        .into_func()
        .expect("get red into func")
        .call(&mut store, &[], &mut result)
        .expect("call get red");
    let red = if let Value::I32(red) = result[0] {
        red as f32 / 255.0
    } else {
        panic!("red is not an I32 value");
    };

    instance
        .get_export(&store, "get_green")
        .expect("get green export")
        .into_func()
        .expect("get green into func")
        .call(&mut store, &[], &mut result)
        .expect("call get green");
    let green = if let Value::I32(green) = result[0] {
        green as f32 / 255.0
    } else {
        panic!("green is not an I32 value");
    };

    instance
        .get_export(&store, "get_blue")
        .expect("get blue export")
        .into_func()
        .expect("get blue into func")
        .call(&mut store, &[], &mut result)
        .expect("call get blue");
    let blue = if let Value::I32(blue) = result[0] {
        blue as f32 / 255.0
    } else {
        panic!("blue is not an I32 value");
    };

    (red, green, blue)
}

#[cfg(not(target_arch = "wasm32"))]
fn get_engine() -> impl backend::WasmEngine {
    wasmtime::Engine::default()
}

#[cfg(target_arch = "wasm32")]
fn get_engine() -> impl backend::WasmEngine {
    web::Engine::default()
}
