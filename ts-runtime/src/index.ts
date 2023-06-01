import { readFileSync } from "fs";
import { ProtocolPlugin } from "../guest-bindings/protocol-plugin.js";
import {
  ProtocolHost,
  addProtocolHostToImports,
} from "../host-bindings/protocol-host.js";

const MODULE_BYTES = readFileSync(
  "../rust-plugin/target/wasm32-unknown-unknown/debug/rust_plugin.wasm"
);

const main = async () => {
  const importObj = {};
  const imports: ProtocolHost = {
    addOneToAll(numbers: Uint8Array) {
      return numbers.map((n) => n + 1);
    },
  };
  let instance: WebAssembly.Instance;
  addProtocolHostToImports(
    importObj,
    imports,
    (name) => instance.exports[name]
  );

  const wasm = new ProtocolPlugin();
  await wasm.instantiate(MODULE_BYTES, importObj);

  instance = wasm.instance;

  console.log(wasm.addThreeToAll(new Uint8Array([5, 8, 17])));
};

main().catch(console.error);
