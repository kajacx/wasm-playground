use sample_protocol_plugin::SampleProtocolPluginData;
use wai_bindgen_wasmtime::wasmtime::{Config, Engine, Linker, Module, Store};

const PLUGIN_BYTES: &'static [u8] = include_bytes!(
    "../../wai-sample-plugin/target/wasm32-unknown-unknown/debug/wai_sample_plugin.wasm"
);

wai_bindgen_wasmtime::import!("../sample-protocol-plugin.wai");
wai_bindgen_wasmtime::export!("../sample-protocol-host.wai");

impl sample_protocol_host::SampleProtocolHost for SampleProtocolPluginData {
    fn add_one(&mut self, num: u32) -> u32 {
        num + 1
    }

    fn move_y(
        &mut self,
        mut vec: sample_protocol_host::Vector3f,
    ) -> sample_protocol_host::Vector3f {
        vec.y += 1.0;
        vec
    }
}

fn main() {
    let config = Config::new();
    let engine = Engine::new(&config).expect("should create engine");

    let mut store = Store::new(&engine, SampleProtocolPluginData::default());

    let mut linker = Linker::<SampleProtocolPluginData>::new(&engine);
    sample_protocol_host::add_to_linker(&mut linker, |data| data).expect("should link host fns");

    let module = Module::new(&engine, PLUGIN_BYTES).expect("should load module from bytes");

    let (plugin, _instance) = sample_protocol_plugin::SampleProtocolPlugin::instantiate(
        &mut store,
        &module,
        &mut linker,
        |data| data,
    )
    .expect("should create instance");

    println!("{:?}", plugin.add_three(&mut store, 5));

    println!(
        "{:?}",
        plugin.move_vec(
            &mut store,
            sample_protocol_plugin::Vector3f {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        )
    );
}
