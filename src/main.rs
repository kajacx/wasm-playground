use js_component_bindgen::{transpile, InstantiationMode, TranspileOpts};

const BYTES: &[u8] = include_bytes!("E:/Programming/Rust/wasm-bridge/tests/instance/guest/target/wasm32-unknown-unknown/release/wit_components_guest.wasm");

fn main() {
    let opts = TranspileOpts {
        name: "component".into(),
        instantiation: Some(InstantiationMode::Async),
        ..Default::default()
    };

    let transpiled = transpile(BYTES, opts).unwrap();
    for (name, content) in transpiled.files {
        println!("Writing {name}");
        std::fs::write(format!("./out/{name}"), content).unwrap();
    }

    println!("All done.")
}
