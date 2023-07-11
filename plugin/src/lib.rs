wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "calculator",
});

struct Impl;

impl Calculator for Impl {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

export_calculator!(Impl);
