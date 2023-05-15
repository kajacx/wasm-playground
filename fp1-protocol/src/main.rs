use std::collections::{BTreeMap, BTreeSet};

use fp_bindgen::{prelude::*, types::CargoDependency};
use types::Vector2f;

mod types;

fp_import! {
    fn add_one_u32(input: u32) -> u32;

    fn set_x_coord(x: f32);
    fn set_y_coord(y: f32);

    fn get_x_coord() -> f32;
    fn get_y_coord() -> f32;

    async fn get_file_contents(path: String) -> Vec<u8>;
}

fp_export! {
    fn add_two_u32(input: u32) -> u32;

    fn move_point(point: Vector2f) -> Vector2f;

    async fn get_data_from_files(amount: u32) -> String;
}

fn main() {
    let dependency = CargoDependency {
        features: BTreeSet::from(["async", "guest"]),
        path: Some("../../../fp-bindgen/fp-bindgen-support"),
        ..Default::default()
    };

    // For plugin
    fp_bindgen!(BindingConfig {
        bindings_type: BindingsType::RustPlugin(RustPluginConfig {
            name: "fp-protocol",
            authors: "[\"kajacx\"]",
            version: "0.1.0",
            dependencies: BTreeMap::from([("fp-bindgen-support", dependency)]),
        }),
        path: "bindings/rust-plugin",
    });

    // For runtime
    fp_bindgen!(BindingConfig {
        bindings_type: BindingsType::RustWasmerRuntime,
        path: "bindings/rust-wasmer-runtime",
    });
}
