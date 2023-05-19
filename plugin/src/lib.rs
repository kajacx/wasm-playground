wai_bindgen_rust::export!("../protocol-plugin.wai");

struct ProtocolPlugin;

impl protocol_plugin::ProtocolPlugin for ProtocolPlugin {
    fn add_three(number: u32) -> u32 {
        number + 3
    }

    fn get_color() -> protocol_plugin::Color {
        protocol_plugin::Color {
            r: 0.1,
            g: 0.5,
            b: 0.0,
        }
    }
}

// #[no_mangle]
// pub fn add_three(number: u32) -> u32 {
//     number.wrapping_add(3)
// }
