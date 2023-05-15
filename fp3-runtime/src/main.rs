use crate::spec::{bindings::Runtime, types::Vector2f, GLOBAL_POINT};

mod spec;

fn get_runtime() -> Runtime {
    let bytes = std::fs::read("../fp2-plugin/target/wasm32-unknown-unknown/debug/fp_plugin.wasm")
        .expect("Should read WASM bytes");

    let runtime = Runtime::new(&bytes).expect("Runtime should be created correctly");

    runtime
}

#[tokio::main]
async fn main() {
    let runtime = get_runtime();

    let result = runtime.add_two_u32(5).expect("Should add two successfully");
    println!("5 + 2 = {result}");

    GLOBAL_POINT.lock().expect("Point lock").x = 5.0;
    GLOBAL_POINT.lock().expect("Point lock").y = 6.0;

    let point = runtime.move_point(Vector2f { x: 10.0, y: 12.0 });
    println!(
        "Point: {:?}, global point: {:?}",
        point,
        GLOBAL_POINT.lock().expect("Point lock")
    );

    let result = runtime.get_data_from_files(4).await.expect("PLS WORK");
    println!("DATA:\n\n{result}");
}

#[test]
fn test() {
    let runtime = get_runtime();

    let result = runtime
        .add_two_u32(10)
        .expect("Should add two successfully");
    assert_eq!(result, 12);
}
