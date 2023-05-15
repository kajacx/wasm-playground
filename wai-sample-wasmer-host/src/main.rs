use sample_protocol_plugin::SampleProtocolPluginData;
use wai_bindgen_wasmer::wasmer::*;

const PLUGIN_BYTES: &'static [u8] = include_bytes!(
    "../../wai-sample-plugin/target/wasm32-unknown-unknown/debug/wai_sample_plugin.wasm"
);

wai_bindgen_wasmer::import!("../sample-protocol-plugin.wai");
wai_bindgen_wasmer::export!("../sample-protocol-host.wai");

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
    let compiler = Cranelift::default();

    let mut store = Store::new(compiler);

    let mut imports = imports! {};
    let add_imports = sample_protocol_host::add_to_imports(
        &mut store,
        &mut imports,
        SampleProtocolPluginData::default(),
    );

    let module = Module::new(&store, PLUGIN_BYTES).expect("should load module from bytes");

    let (plugin, instance) = sample_protocol_plugin::SampleProtocolPlugin::instantiate(
        &mut store,
        &module,
        &mut imports,
    )
    .expect("should create instance");

    add_imports(&instance, &store).expect("should add imports");

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
