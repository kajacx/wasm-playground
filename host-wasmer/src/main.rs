use std::sync::{Arc, Mutex};

use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use protocol_host::{add_to_imports, ProtocolHost};
use wai_bindgen_wasmer::wasmer::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/bevy_plugin.wasm");

wai_bindgen_wasmer::import!("../protocol-plugin.wai");
wai_bindgen_wasmer::export!("../protocol-host.wai");

struct MyData {
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
    // let bevy_color = Color::rgb(0.2, 0.5, 0.9);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .insert_resource(ClearColor(bevy_color))
        .add_startup_system(setup)
        .add_system(print_color)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn get_color() -> protocol_plugin::Color {
    let mut store = Store::new(Engine::default());

    let module = Module::new(&store, PLUGIN_BYTES).expect("should create module");

    let red = Arc::new(Mutex::new(0.8f32));
    let my_data = MyData { red: red.clone() };

    let mut imports = imports! {};
    let adder = add_to_imports(&mut store, &mut imports, my_data);

    let (plugin, instance) =
        &protocol_plugin::ProtocolPlugin::instantiate(&mut store, &module, &mut imports)
            .expect("should create instance");

    adder(instance, &store).expect("should add imports");

    plugin.get_color(&mut store).expect("should get color")
}

fn print_color() {
    info!("COLOR IS: {:?}", get_color());
}
