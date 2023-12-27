cargo_component_bindings::generate!();

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        println!("STDOUT: Hello, World!");
        "RETURN: Hello, World!".to_string()
    }
}
