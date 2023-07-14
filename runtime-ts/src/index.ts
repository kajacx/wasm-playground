import { readFileSync } from "node:fs";

let bytes = readFileSync(
  "../plugin/target/wasm32-unknown-unknown/debug/wasmtime_plugin.wasm"
);

WebAssembly.instantiate(bytes, {
  imported_fns: {
    add_one_i32(num: number) {
      return num + 1;
    },
    add_one_pair(a: number, b: number) {
      return [a + 1, b + 1];
    },
  },
}).then((wasm) => {
  console.log("Add three", (wasm as any).instance.exports.add_three_i32(5));
  console.log("Add", (wasm as any).instance.exports.add_i32(5, 10));
  console.log(
    "Add sub ten",
    (wasm as any).instance.exports.add_sub_ten_i32(50)
  );
  console.log(
    "Add three pair",
    (wasm as any).instance.exports.add_three_pair(5, 15)
  );
});
