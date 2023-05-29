wai_bindgen_rust::export!("../protocol-plugin.wai");
wai_bindgen_rust::import!("../protocol-host.wai");

use protocol_host::*;
use protocol_plugin::*;

struct ProtocolPlugin;

impl protocol_plugin::ProtocolPlugin for ProtocolPlugin {
    fn get_color() -> Color {
        Color {
            r: get_red(),
            g: 0.8,
            b: 0.3,
        }
    }
}
