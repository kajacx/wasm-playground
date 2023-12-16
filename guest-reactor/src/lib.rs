cargo_component_bindings::generate!();

use bindings::{Guest, Shape};

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }

    fn double_shape(input: Shape) -> Shape {
        match input {
            Shape::Circle(r) => Shape::Circle(r * 2.0),
            _ => Shape::Point,
        }
    }
}
