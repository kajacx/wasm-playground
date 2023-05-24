wai_bindgen_rust::export!("../protocol-plugin.wai");

use protocol_plugin::*;

struct ProtocolPlugin;

impl protocol_plugin::ProtocolPlugin for ProtocolPlugin {
    fn get_color() -> Color {
        Color {
            r: 0.1,
            g: 0.8,
            b: 0.3,
        }
    }
}
