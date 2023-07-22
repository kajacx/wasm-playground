wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "calculator",
});

struct Impl;

impl Calculator for Impl {
    fn add_three(num: i32) -> i32 {
        let num = add_one(num);
        let num = add_one(num);
        let num = add_one(num);
        num
    }
}

export_calculator!(Impl);
