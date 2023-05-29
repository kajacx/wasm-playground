use std::sync::{Arc, Mutex};

use bevy::prelude::Color;
use bevy::prelude::*;
use wai_bindgen_wasmtime::wasmtime::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/bevy_plugin.wasm");

wai_bindgen_wasmtime::import!("../protocol-plugin.wai");
wai_bindgen_wasmtime::export!("../protocol-host.wai");

use protocol_host::*;
use protocol_plugin::*;

struct MyData {
    data: ProtocolPluginData,
    red: Arc<Mutex<f32>>,
}

impl ProtocolHost for MyData {
    fn get_red(&mut self) -> f32 {
        *self.red.try_lock().expect("should lock mutex")
    }
}

fn main() {
    let plugin_color = get_color();
    let bevy_color = Color::rgb(plugin_color.r, plugin_color.g, plugin_color.b);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(bevy_color))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn get_color() -> protocol_plugin::Color {
    let config = Config::new();
    let engine = Engine::new(&config).expect("should create engine");

    let mut linker = Linker::<MyData>::new(&engine);
    add_to_linker(&mut linker, |data| data).expect("should add to linker");

    let module = Module::new(&engine, PLUGIN_BYTES).expect("should load module from bytes");

    let red = Arc::new(Mutex::new(0.8f32));
    let mut store = Store::new(
        &engine,
        MyData {
            data: ProtocolPluginData::default(),
            red: red.clone(),
        },
    );

    let (plugin, _instance) =
        ProtocolPlugin::instantiate(&mut store, &module, &mut linker, |data| &mut data.data)
            .expect("should create instance");

    plugin.get_color(&mut store).expect("should get color")
}
