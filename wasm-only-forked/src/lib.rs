#![feature(core_intrinsics)]
#![feature(structural_match)]
#![feature(no_coverage)]
#![feature(derive_clone_copy)]

use wai_bindgen_wasmer::wasmer::*;
use wasm_bindgen::prelude::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/release/bevy_plugin.wasm");

wai_bindgen_wasmer::import!("../protocol-plugin.wai");

// #[allow(clippy::all)]
// pub mod protocol_plugin {
//     #[allow(unused_imports)]
//     use wai_bindgen_wasmer::{anyhow, wasmer};
//     /// Auxiliary data associated with the wasm exports.
//     pub struct ProtocolPluginData {}
//     #[automatically_derived]
//     impl ::core::default::Default for ProtocolPluginData {
//         #[inline]
//         fn default() -> ProtocolPluginData {
//             ProtocolPluginData {}
//         }
//     }
//     pub struct ProtocolPlugin {
//         #[allow(dead_code)]
//         env: wasmer::FunctionEnv<ProtocolPluginData>,
//         func_add_three: wasmer::TypedFunction<i32, i32>,
//     }
//     impl ProtocolPlugin {
//         #[allow(unused_variables)]
//         /// Adds any intrinsics, if necessary for this exported wasm
//         /// functionality to the `ImportObject` provided.
//         ///
//         /// This function returns the `ProtocolPluginData` which needs to be
//         /// passed through to `ProtocolPlugin::new`.
//         fn add_to_imports(
//             mut store: impl wasmer::AsStoreMut,
//             imports: &mut wasmer::Imports,
//         ) -> wasmer::FunctionEnv<ProtocolPluginData> {
//             let env = wasmer::FunctionEnv::new(&mut store, ProtocolPluginData::default());
//             env
//         }
//         /// Instantiates the provided `module` using the specified
//         /// parameters, wrapping up the result in a structure that
//         /// translates between wasm and the host.
//         ///
//         /// The `imports` provided will have intrinsics added to it
//         /// automatically, so it's not necessary to call
//         /// `add_to_imports` beforehand. This function will
//         /// instantiate the `module` otherwise using `imports`, and
//         /// both an instance of this structure and the underlying
//         /// `wasmer::Instance` will be returned.
//         pub fn instantiate(
//             mut store: impl wasmer::AsStoreMut,
//             module: &wasmer::Module,
//             imports: &mut wasmer::Imports,
//         ) -> anyhow::Result<(Self, wasmer::Instance)> {
//             let env = Self::add_to_imports(&mut store, imports);
//             let instance = wasmer::Instance::new(&mut store, module, &*imports)?;
//             Ok((Self::new(store, &instance, env)?, instance))
//         }
//         /// Low-level creation wrapper for wrapping up the exports
//         /// of the `instance` provided in this structure of wasm
//         /// exports.
//         ///
//         /// This function will extract exports from the `instance`
//         /// and wrap them all up in the returned structure which can
//         /// be used to interact with the wasm module.
//         pub fn new(
//             store: impl wasmer::AsStoreMut,
//             _instance: &wasmer::Instance,
//             env: wasmer::FunctionEnv<ProtocolPluginData>,
//         ) -> Result<Self, wasmer::ExportError> {
//             let func_add_three = _instance
//                 .exports
//                 .get_function("add-three")
//                 .expect("GET IT")
//                 .typed(&store)
//                 .unwrap_or_else(|e| panic!("{e:?}"));
//             Ok(ProtocolPlugin {
//                 func_add_three,
//                 env,
//             })
//         }
//         pub fn add_three(
//             &self,
//             store: &mut wasmer::Store,
//             number: u32,
//         ) -> Result<u32, wasmer::RuntimeError> {
//             let result0 = self
//                 .func_add_three
//                 .call(store, wai_bindgen_wasmer::rt::as_i32(number))?;
//             Ok(result0 as u32)
//         }
//     }
//     #[allow(unused_imports)]
//     use wasmer::AsStoreMut as _;
//     #[allow(unused_imports)]
//     use wasmer::AsStoreRef as _;
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn return_eight() -> i32 {
    console_error_panic_hook::set_once();
    // alert(&format!("5 + 3 = {:?}", add_three(5)));
    add_three(5) as _
}

#[wasm_bindgen]
pub fn return_color() -> String {
    console_error_panic_hook::set_once();
    // alert(&format!("5 + 3 = {:?}", add_three(5)));
    format!("{:?}", get_color())
}

fn add_three(number: u32) -> u32 {
    let (mut store, module) = create_store();

    let (plugin, _) =
        protocol_plugin::ProtocolPlugin::instantiate(&mut store, &module, &mut imports! {})
            .expect("should create instance");

    plugin
        .add_three(&mut store, number)
        .expect("should add three")
}

fn get_color() -> protocol_plugin::Color {
    let (mut store, module) = create_store();

    let (plugin, _) =
        protocol_plugin::ProtocolPlugin::instantiate(&mut store, &module, &mut imports! {})
            .expect("should create instance");

    plugin.get_color(&mut store).expect("should get color")
}

fn create_store() -> (Store, Module) {
    let store = Store::new(Engine::default());
    //Store::new()

    // let bytes = std::fs::read("../../plugin/target/wasm32-unknown-unknown/debug/bevy_plugin.wasm")
    //     .expect("should read bytes");
    let bytes = PLUGIN_BYTES;
    let module = Module::new(&store, &bytes).expect("should create module");

    (store, module)
}
