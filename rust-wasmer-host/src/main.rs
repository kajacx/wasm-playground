use wai_bindgen_wasmer::wasmer::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../rust-plugin/target/wasm32-unknown-unknown/debug/rust_plugin.wasm");

wai_bindgen_wasmer::import!("../protocol-plugin.wai");
wai_bindgen_wasmer::export!("../protocol-host.wai");

struct HostData;

impl protocol_host::ProtocolHost for HostData {
    fn add_one_to_all(&mut self, numbers: &[u8]) -> Vec<u8> {
        numbers.iter().map(|x| *x + 1).collect()
    }
}

fn main() {
    let mut store = Store::default();

    let mut imports = imports! {};
    let add_imports = protocol_host::add_to_imports(&mut store, &mut imports, HostData);

    let module = Module::new(&store, PLUGIN_BYTES).expect("should load module from bytes");

    let (plugin, instance) =
        protocol_plugin::ProtocolPlugin::instantiate(&mut store, &module, &mut imports)
            .expect("should create instance");

    add_imports(&instance, &store).expect("should add imports");

    println!("{:?}", plugin.add_three_to_all(&mut store, &[5, 8, 6]));
}
