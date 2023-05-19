wai_bindgen_rust::export!("../protocol-plugin.wai");

struct ProtocolPlugin;

impl protocol_plugin::ProtocolPlugin for ProtocolPlugin {
    fn add_three(number: u32) -> u32 {
        number + 3
    }
}

// #[no_mangle]
// pub fn add_three(number: u32) -> u32 {
//     number.wrapping_add(3)
// }
