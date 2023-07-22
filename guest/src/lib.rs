wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "calculator",
});

struct Impl;

impl Calculator for Impl {
    fn add_three(num: i32) -> i32 {
        num + 3
    }
}

export_calculator!(Impl);
