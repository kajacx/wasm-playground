use protocol_host::add_one_to_all;

wai_bindgen_rust::export!("../protocol-plugin.wai");
wai_bindgen_rust::import!("../protocol-host.wai");

struct ProtocolPlugin;

impl protocol_plugin::ProtocolPlugin for ProtocolPlugin {
    fn add_three_to_all(mut numbers: Vec<u8>) -> Vec<u8> {
        numbers.iter_mut().for_each(|x| *x = *x + 1);
        let mut numbers = add_one_to_all(&numbers);
        numbers.iter_mut().for_each(|x| *x = *x + 1);
        numbers
    }
}
