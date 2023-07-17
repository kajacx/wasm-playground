let dv = new DataView(new ArrayBuffer());
const dataView = (mem) =>
  dv.buffer === mem.buffer ? dv : (dv = new DataView(mem.buffer));

function toInt32(val) {
  return val >> 0;
}

const utf8Decoder = new TextDecoder();

const isNode =
  typeof process !== "undefined" && process.versions && process.versions.node;
let _fs;
async function fetchCompile(url) {
  if (isNode) {
    _fs = _fs || (await import("fs/promises"));
    return WebAssembly.compile(await _fs.readFile(url));
  }
  return fetch(url).then(WebAssembly.compileStreaming);
}

// TODO: should be Promise.all
const module0 = await fetchCompile(
  new URL("./component.core.wasm", import.meta.url)
);
const module1 = await fetchCompile(
  new URL("./component.core2.wasm", import.meta.url)
);
const module2 = await fetchCompile(
  new URL("./component.core3.wasm", import.meta.url)
);

export async function instantiate(
  // compileCore,
  imports,
  instantiateCore = WebAssembly.instantiate
) {
  const importPoint = imports["import-point"].default;
  const print = imports.print.default;
  let exports0;
  let exports1;
  let memory0;

  function lowering0(arg0, arg1) {
    const ptr0 = arg0;
    const len0 = arg1;
    const result0 = utf8Decoder.decode(
      new Uint8Array(memory0.buffer, ptr0, len0)
    );
    print(result0);
  }

  function lowering1(arg0, arg1, arg2) {
    const ret = importPoint({
      x: arg0,
      y: arg1,
    });
    const { x: v0_0, y: v0_1 } = ret;
    dataView(memory0).setInt32(arg2 + 0, toInt32(v0_0), true);
    dataView(memory0).setInt32(arg2 + 4, toInt32(v0_1), true);
  }
  let exports2;
  Promise.all([module0, module1, module2]).catch(() => {});
  ({ exports: exports0 } = await instantiateCore(await module1));
  ({ exports: exports1 } = await instantiateCore(await module0, {
    $root: {
      "import-point": exports0["1"],
      print: exports0["0"],
    },
  }));
  memory0 = exports1.memory;
  ({ exports: exports2 } = await instantiateCore(await module2, {
    "": {
      $imports: exports0.$imports,
      0: lowering0,
      1: lowering1,
    },
  }));

  function run() {
    exports1.run();
  }

  function movePoint(arg0) {
    const { x: v0_0, y: v0_1 } = arg0;
    const ret = exports1["move-point"](toInt32(v0_0), toInt32(v0_1));
    return {
      x: dataView(memory0).getInt32(ret + 0, true),
      y: dataView(memory0).getInt32(ret + 4, true),
    };
  }

  function increment() {
    const ret = exports1.increment();
    return ret;
  }

  return { increment, movePoint, run };
}
