use sample_protocol_host::{add_one, move_y};

wai_bindgen_rust::export!("../sample-protocol-plugin.wai");
wai_bindgen_rust::import!("../sample-protocol-host.wai");

struct SampleProtocolPlugin;

impl sample_protocol_plugin::SampleProtocolPlugin for SampleProtocolPlugin {
    fn add_three(num: u32) -> u32 {
        add_one(num + 1) + 1
    }

    fn move_vec(vec: sample_protocol_plugin::Vector3f) -> sample_protocol_plugin::Vector3f {
        let mut as_host = sample_protocol_host::Vector3f {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        };

        as_host.x += 1.0;

        as_host = move_y(as_host);

        as_host.z += 1.0;

        sample_protocol_plugin::Vector3f {
            x: as_host.x,
            y: as_host.y,
            z: as_host.z,
        }
    }
}
