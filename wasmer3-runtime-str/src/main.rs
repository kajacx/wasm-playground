use std::{
    collections::HashMap,
    str,
    sync::{Arc, Mutex},
};
use wasmer::*;

struct AddressBook(HashMap<String, f64>);

impl AddressBook {
    fn get_balance(&self, address: &str) -> f64 {
        match self.0.get(address) {
            Some(val) => *val,
            None => 0.0,
        }
    }
}

struct Env {
    address_book: Arc<Mutex<AddressBook>>,
    memory: Arc<Mutex<Option<Memory>>>,
}

fn main() {
    let wasm_bytes = include_bytes!(
        "../../wasmer-plugin-str/target/wasm32-unknown-unknown/debug/wasmer_plugin.wasm"
    )
    .as_ref();

    // Create the store
    let mut store = Store::new(Engine::default());

    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes).expect("should instantiate module");

    let mut address_book = AddressBook(HashMap::new());
    address_book.0.insert("account1".into(), 11.0);
    address_book.0.insert("account2".into(), 12.0);
    address_book.0.insert("account3".into(), 13.0);

    let address_book = Arc::new(Mutex::new(address_book));

    let memory_cell = Arc::new(Mutex::new(None));

    let env = FunctionEnv::new(
        &mut store,
        Env {
            address_book: address_book.clone(),
            memory: memory_cell.clone(),
        },
    );

    // Create an empty import object.
    let import_object = imports! {
        "my_imports" => {
            "get_balance_raw" => Function::new_typed_with_env(&mut store, &env, |envf: FunctionEnvMut<Env>, address_ptr: u32| {
                let data = envf.data();

                // Dark magic to get module's memory
                let memory = data.memory.try_lock().expect("should lock memory").as_ref().expect("should find memory").clone();
                let memory_view = memory.view(&envf);
                let raw_memory = unsafe { memory_view.data_unchecked() };

                // Convert C pointer to Rust str
                let address = address_ptr as usize;
                let mut end_index = address;
                while end_index < raw_memory.len() && raw_memory[end_index] != 0 {
                    end_index = end_index + 1;
                }
                if end_index >= raw_memory.len() {
                    panic!("Did not find ending null byte");
                }

                let customer_address = str::from_utf8(&raw_memory[address..end_index]).expect("should be valid utf-8");

                // Get our data
                let address_book = &data.address_book;

                // Actual user code
                address_book.try_lock().expect("should lock address_book").get_balance(customer_address)
            })
        }
    };

    // Let's instantiate the Wasm module
    let instance = Instance::new(&mut store, &module, &import_object).expect("should get instance");

    // Fill the memory in Env so we can use it later
    *memory_cell.try_lock().expect("should lock memory") = Some(
        instance
            .exports
            .get_memory("memory")
            .expect("should get memory")
            .clone(),
    );

    // Get the exported function
    let do_accounting = instance
        .exports
        .get_typed_function::<(), f64>(&store, "do_accounting")
        .expect("should get do_accounting function");

    // And call it!
    let amount = do_accounting.call(&mut store).expect("should get amount");
    println!("Calculated amount: {amount}");

    // We can even edit the address book now
    address_book
        .try_lock()
        .expect("should lock")
        .0
        .insert("account1".into(), 111.0);

    // And then call the exported method again
    let amount = do_accounting.call(&mut store).expect("should get amount");
    println!("Calculated amount again: {amount}");
}
