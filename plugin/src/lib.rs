wai_bindgen_rust::export!("../protocol-plugin.wai");

use protocol_plugin::*;
struct ProtocolPlugin;

impl protocol_plugin::ProtocolPlugin for ProtocolPlugin {
    fn add_three(number: u32) -> u32 {
        number + 3
    }

    fn get_color() -> Color {
        Color {
            r: 0.2,
            g: 1.0,
            b: 0.4,
        }
    }

    fn get_complex() -> Complex {
        Complex {
            name: "Hello".into(),
            colors: vec![Self::get_color()],
            // my: MyEnum::One,
            my: vec![],
            mys: vec![],
        }
    }
}
