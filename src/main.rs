use js_component_bindgen::*;

// fn main() {
//     let bytes = include_bytes!("E:/Programming/Rust/wasm-bridge/tests/instance/guest/target/wasm32-unknown-unknown/release/wit_components_guest.wasm");

//     let opts = TranspileOpts {
//         name: "wit_components_guest".to_string(),
//         instantiation: Some(InstantiationMode::Async), // TODO: check this
//         ..Default::default()
//     };

//     let transpiled = transpile(bytes, opts).unwrap();
//     let files = transpiled.files;

//     for (name, bytes) in files.into_iter() {
//         println!("WRITING: {name}");
//         std::fs::write(format!("./wit-dir/{name}"), bytes).unwrap();
//     }
// }

fn main() {
    let bytes = include_bytes!("E:/Programming/Rust/wasm-bridge/tests/instance/guest/target/wasm32-wasi/release/wasi_components_guest.wasm");

    let opts = TranspileOpts {
        name: "wasi_components_guest".to_string(),
        instantiation: Some(InstantiationMode::Async), // TODO: check this
        ..Default::default()
    };

    let transpiled = transpile(bytes, opts).unwrap();
    let files = transpiled.files;

    for (name, bytes) in files.into_iter() {
        println!("WRITING: {name}");
        std::fs::write(format!("./wasi-dir/{name}"), bytes).unwrap();
    }
}
