cargo_component_bindings::generate!();

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        let imported = bindings::hello_import();
        println!("STDOUT: Imported: {imported}");
        format!("RETURN: Imported: {imported}")
    }
}
