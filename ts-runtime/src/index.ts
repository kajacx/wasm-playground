import { readFileSync } from "node:fs";

let bytes = readFileSync(
  "../plugin/target/wasm32-unknown-unknown/debug/wasmtime_plugin.wasm"
);

WebAssembly.instantiate(bytes, {
  imported_fns: {
    add_one_i32(num: number) {
      return num + 1;
    },
  },
}).then((wasm) => {
  console.log("Hello", wasm, (wasm as any).instance.exports.add_three_i32(5));
});
