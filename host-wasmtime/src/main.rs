use bevy::prelude::*;
use wai_bindgen_wasmtime::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/wai_sample_plugin.wasm");

wai_bindgen_wasmtime::import!("../protocol-plugin.wai");

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

    let mut linker = Linker::<SampleProtocolPluginData>::new(&engine);
    // sample_protocol_host::add_to_linker(&mut linker, |data| data).expect("should link host fns");

    let module = Module::new(&engine, PLUGIN_BYTES).expect("should load module from bytes");

    let (plugin, _instance) = sample_protocol_plugin::SampleProtocolPlugin::instantiate(
        &mut store,
        &module,
        &mut linker,
        |data| data,
    )
    .expect("should create instance");

    plugin.get_color(&mut store).expect("should get color")
}
