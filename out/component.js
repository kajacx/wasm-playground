let curResourceBorrows = [];

let dv = new DataView(new ArrayBuffer());
const dataView = (mem) =>
  dv.buffer === mem.buffer ? dv : (dv = new DataView(mem.buffer));

const emptyFunc = () => {};

function finalizationRegistryCreate(unregister) {
  if (typeof FinalizationRegistry === "undefined") {
    return { unregister() {} };
  }
  return new FinalizationRegistry(unregister);
}

const handleTables = [];

const T_FLAG = 1 << 30;

function rscTableCreateOwn(table, rep) {
  const free = table[0] & ~T_FLAG;
  if (free === 0) {
    table.push(0);
    table.push(rep | T_FLAG);
    return (table.length >> 1) - 1;
  }
  table[0] = table[free << 1];
  table[free << 1] = 0;
  table[(free << 1) + 1] = rep | T_FLAG;
  return free;
}

function rscTableRemove(table, handle) {
  const scope = table[handle << 1];
  const val = table[(handle << 1) + 1];
  const own = (val & T_FLAG) !== 0;
  const rep = val & ~T_FLAG;
  if (val === 0 || (scope & T_FLAG) !== 0)
    throw new TypeError("Invalid handle");
  table[handle << 1] = table[0] | T_FLAG;
  table[0] = handle | T_FLAG;
  return { rep, scope, own };
}

const symbolCabiDispose = Symbol.for("cabiDispose");

const symbolRscHandle = Symbol("handle");

const symbolRscRep = Symbol.for("cabiRep");

const symbolDispose = Symbol.dispose || Symbol.for("dispose");

function toUint32(val) {
  return val >>> 0;
}

const utf8Decoder = new TextDecoder();

const utf8Encoder = new TextEncoder();

let utf8EncodedLen = 0;
function utf8Encode(s, realloc, memory) {
  if (typeof s !== "string") throw new TypeError("expected a string");
  if (s.length === 0) {
    utf8EncodedLen = 0;
    return 1;
  }
  let allocLen = 0;
  let ptr = 0;
  let writtenTotal = 0;
  while (s.length > 0) {
    ptr = realloc(ptr, allocLen, 1, (allocLen += s.length * 2));
    const { read, written } = utf8Encoder.encodeInto(
      s,
      new Uint8Array(memory.buffer, ptr + writtenTotal, allocLen - writtenTotal)
    );
    writtenTotal += written;
    s = s.slice(read);
  }
  utf8EncodedLen = writtenTotal;
  return ptr;
}

export async function instantiate(
  getCoreModule,
  imports,
  instantiateCore = WebAssembly.instantiate
) {
  const module0 = getCoreModule("component.core.wasm");
  const module1 = getCoreModule("component.core2.wasm");
  const module2 = getCoreModule("component.core3.wasm");

  const { Company } = imports["component-test:wit-protocol/companies"];
  const { companyRoundtrip } = imports["component-test:wit-protocol/host-fns"];
  let exports0;
  const handleTable0 = [T_FLAG, 0];
  const captureTable0 = new Map();
  let captureCnt0 = 0;
  handleTables[0] = handleTable0;

  function trampoline2(arg0) {
    var handle1 = arg0;
    var rep2 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
    var rsc0 = captureTable0.get(rep2);
    if (!rsc0) {
      rsc0 = Object.create(Company.prototype);
      Object.defineProperty(rsc0, symbolRscHandle, {
        writable: true,
        value: handle1,
      });
      Object.defineProperty(rsc0, symbolRscRep, {
        writable: true,
        value: rep2,
      });
    } else {
      captureTable0.delete(rep2);
    }
    rscTableRemove(handleTable0, handle1);
    const ret = companyRoundtrip(rsc0);
    if (!(ret instanceof Company)) {
      throw new TypeError('Resource error: Not a valid "Company" resource.');
    }
    var handle3 = ret[symbolRscHandle];
    if (!handle3) {
      const rep = ret[symbolRscRep] || ++captureCnt0;
      captureTable0.set(rep, ret);
      handle3 = rscTableCreateOwn(handleTable0, rep);
    }
    return handle3;
  }

  function trampoline5(arg0) {
    var handle1 = arg0;
    var rep2 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
    var rsc0 = captureTable0.get(rep2);
    if (!rsc0) {
      rsc0 = Object.create(Company.prototype);
      Object.defineProperty(rsc0, symbolRscHandle, {
        writable: true,
        value: handle1,
      });
      Object.defineProperty(rsc0, symbolRscRep, {
        writable: true,
        value: rep2,
      });
    }
    curResourceBorrows.push(rsc0);
    const ret = rsc0.getMaxSalary();
    for (const rsc of curResourceBorrows) {
      rsc[symbolRscHandle] = null;
    }
    curResourceBorrows = [];
    return toUint32(ret);
  }
  let exports1;
  let exports2;
  let memory0;
  let realloc0;
  let postReturn0;
  const handleTable1 = [T_FLAG, 0];
  const finalizationRegistry1 = finalizationRegistryCreate((handle) => {
    const { rep } = rscTableRemove(handleTable1, handle);
    exports0["0"](rep);
  });

  handleTables[1] = handleTable1;
  const trampoline0 = rscTableCreateOwn.bind(null, handleTable1);
  function trampoline1(handle) {
    const handleEntry = rscTableRemove(handleTable1, handle);
    if (handleEntry.own) {
      exports0["0"](handleEntry.rep);
    }
  }
  function trampoline3(handle) {
    const handleEntry = rscTableRemove(handleTable0, handle);
    if (handleEntry.own) {
      const rsc = captureTable0.get(handleEntry.rep);
      if (rsc) {
        if (rsc[symbolDispose]) rsc[symbolDispose]();
        captureTable0.delete(handleEntry.rep);
      } else if (Company[symbolCabiDispose]) {
        Company[symbolCabiDispose](handleEntry.rep);
      }
    }
  }
  function trampoline4(handle) {
    return handleTable1[(handle << 1) + 1] & ~T_FLAG;
  }

  const withLogging =
    (callback, name) =>
    (...args) => {
      let result = callback(...args);
      console.log(
        `Function '${name}' returned`,
        result,
        "with arguments:",
        ...args
      );
      return result;
    };

  Promise.all([module0, module1, module2]).catch(() => {});
  ({ exports: exports0 } = await instantiateCore(await module1));
  ({ exports: exports1 } = await instantiateCore(await module0, {
    "[export]component-test:wit-protocol/employees": {
      "[resource-drop]employee": withLogging(trampoline1),
      "[resource-new]employee": withLogging(trampoline0),
      "[resource-rep]employee": withLogging(trampoline2),
    },
    "component-test:wit-protocol/companies": {
      "[method]company.get-max-salary": trampoline5,
      "[resource-drop]company": trampoline3,
    },
    "component-test:wit-protocol/host-fns": {
      "company-roundtrip": trampoline2,
    },
  }));
  ({ exports: exports2 } = await instantiateCore(await module2, {
    "": {
      $imports: exports0.$imports,
      0: exports1["component-test:wit-protocol/employees#[dtor]employee"],
    },
  }));
  memory0 = exports1.memory;
  realloc0 = exports1.cabi_realloc;
  postReturn0 =
    exports1[
      "cabi_post_component-test:wit-protocol/employees#[method]employee.get-name"
    ];

  class Employee {
    constructor(arg0, arg1) {
      var ptr0 = utf8Encode(arg0, realloc0, memory0);
      var len0 = utf8EncodedLen;
      const ret = exports1[
        "component-test:wit-protocol/employees#[constructor]employee"
      ](ptr0, len0, toUint32(arg1));
      var handle2 = ret;
      var rsc1 =
        new.target === Employee ? this : Object.create(Employee.prototype);
      Object.defineProperty(rsc1, symbolRscHandle, {
        writable: true,
        value: handle2,
      });
      finalizationRegistry1.register(rsc1, handle2, rsc1);
      Object.defineProperty(rsc1, symbolDispose, {
        writable: true,
        value: function () {
          finalizationRegistry1.unregister(rsc1);
          rscTableRemove(handleTable1, handle2);
          rsc1[symbolDispose] = emptyFunc;
          rsc1[symbolRscHandle] = null;
          exports0["0"](handleTable1[(handle2 << 1) + 1] & ~T_FLAG);
        },
      });
      return rsc1;
    }
  }

  Employee.prototype.getName = function getName() {
    var handle1 = this[symbolRscHandle];
    if (!handle1 || (handleTable1[(handle1 << 1) + 1] & T_FLAG) === 0) {
      throw new TypeError('Resource error: Not a valid "Employee" resource.');
    }
    var handle0 = handleTable1[(handle1 << 1) + 1] & ~T_FLAG;
    const ret =
      exports1[
        "component-test:wit-protocol/employees#[method]employee.get-name"
      ](handle0);
    var ptr2 = dataView(memory0).getInt32(ret + 0, true);
    var len2 = dataView(memory0).getInt32(ret + 4, true);
    var result2 = utf8Decoder.decode(
      new Uint8Array(memory0.buffer, ptr2, len2)
    );
    postReturn0(ret);
    return result2;
  };

  Employee.prototype.getMinSalary = function getMinSalary() {
    var handle1 = this[symbolRscHandle];
    if (!handle1 || (handleTable1[(handle1 << 1) + 1] & T_FLAG) === 0) {
      throw new TypeError('Resource error: Not a valid "Employee" resource.');
    }
    var handle0 = handleTable1[(handle1 << 1) + 1] & ~T_FLAG;
    const ret =
      exports1[
        "component-test:wit-protocol/employees#[method]employee.get-min-salary"
      ](handle0);
    return ret >>> 0;
  };

  function employeeRoundtrip(arg0) {
    var handle0 = arg0[symbolRscHandle];
    if (!handle0) {
      throw new TypeError('Resource error: Not a valid "Employee" resource.');
    }
    finalizationRegistry1.unregister(arg0);
    arg0[symbolDispose] = emptyFunc;
    arg0[symbolRscHandle] = null;
    const ret =
      exports1["component-test:wit-protocol/guest-fns#employee-roundtrip"](
        handle0
      );
    var handle2 = ret;
    var rsc1 =
      new.target === Employee ? this : Object.create(Employee.prototype);
    Object.defineProperty(rsc1, symbolRscHandle, {
      writable: true,
      value: handle2,
    });
    finalizationRegistry1.register(rsc1, handle2, rsc1);
    Object.defineProperty(rsc1, symbolDispose, {
      writable: true,
      value: function () {
        finalizationRegistry1.unregister(rsc1);
        rscTableRemove(handleTable1, handle2);
        rsc1[symbolDispose] = emptyFunc;
        rsc1[symbolRscHandle] = null;
        exports0["0"](handleTable1[(handle2 << 1) + 1] & ~T_FLAG);
      },
    });
    return rsc1;
  }

  function companyRoundtrip$1(arg0) {
    if (!(arg0 instanceof Company)) {
      throw new TypeError('Resource error: Not a valid "Company" resource.');
    }
    var handle0 = arg0[symbolRscHandle];
    if (!handle0) {
      const rep = arg0[symbolRscRep] || ++captureCnt0;
      captureTable0.set(rep, arg0);
      handle0 = rscTableCreateOwn(handleTable0, rep);
    }
    const ret =
      exports1["component-test:wit-protocol/guest-fns#company-roundtrip"](
        handle0
      );
    var handle2 = ret;
    var rep3 = handleTable0[(handle2 << 1) + 1] & ~T_FLAG;
    var rsc1 = captureTable0.get(rep3);
    if (!rsc1) {
      rsc1 = Object.create(Company.prototype);
      Object.defineProperty(rsc1, symbolRscHandle, {
        writable: true,
        value: handle2,
      });
      Object.defineProperty(rsc1, symbolRscRep, {
        writable: true,
        value: rep3,
      });
    } else {
      captureTable0.delete(rep3);
    }
    rscTableRemove(handleTable0, handle2);
    return rsc1;
  }

  function findJob(arg0, arg1) {
    var handle0 = arg0[symbolRscHandle];
    if (!handle0) {
      throw new TypeError('Resource error: Not a valid "Employee" resource.');
    }
    finalizationRegistry1.unregister(arg0);
    arg0[symbolDispose] = emptyFunc;
    arg0[symbolRscHandle] = null;
    var vec2 = arg1;
    var len2 = vec2.length;
    var result2 = realloc0(0, 0, 4, len2 * 4);
    for (let i = 0; i < vec2.length; i++) {
      const e = vec2[i];
      const base = result2 + i * 4;
      if (!(e instanceof Company)) {
        throw new TypeError('Resource error: Not a valid "Company" resource.');
      }
      var handle1 = e[symbolRscHandle];
      if (!handle1) {
        const rep = e[symbolRscRep] || ++captureCnt0;
        captureTable0.set(rep, e);
        handle1 = rscTableCreateOwn(handleTable0, rep);
      }
      dataView(memory0).setInt32(base + 0, handle1, true);
    }
    const ret = exports1["component-test:wit-protocol/guest-fns#find-job"](
      handle0,
      result2,
      len2
    );
    let variant6;
    switch (dataView(memory0).getUint8(ret + 0, true)) {
      case 0: {
        variant6 = undefined;
        break;
      }
      case 1: {
        var handle4 = dataView(memory0).getInt32(ret + 4, true);
        var rep5 = handleTable0[(handle4 << 1) + 1] & ~T_FLAG;
        var rsc3 = captureTable0.get(rep5);
        if (!rsc3) {
          rsc3 = Object.create(Company.prototype);
          Object.defineProperty(rsc3, symbolRscHandle, {
            writable: true,
            value: handle4,
          });
          Object.defineProperty(rsc3, symbolRscRep, {
            writable: true,
            value: rep5,
          });
        } else {
          captureTable0.delete(rep5);
        }
        rscTableRemove(handleTable0, handle4);
        variant6 = rsc3;
        break;
      }
      default: {
        throw new TypeError("invalid variant discriminant for option");
      }
    }
    return variant6;
  }
  const employees = {
    Employee: Employee,
  };
  const guestFns = {
    companyRoundtrip: companyRoundtrip$1,
    employeeRoundtrip: employeeRoundtrip,
    findJob: findJob,
  };

  return {
    employees,
    guestFns,
    "component-test:wit-protocol/employees": employees,
    "component-test:wit-protocol/guest-fns": guestFns,
  };
}
