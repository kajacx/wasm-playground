use std::sync::Mutex;

use self::types::Vector2f;

pub mod bindings;
pub mod types;

pub static GLOBAL_POINT: Mutex<Vector2f> = Mutex::new(Vector2f { x: 0.0, y: 0.0 });

fn add_one_u32(input: u32) -> u32 {
    input + 1
}

fn set_x_coord(coord: f32) {
    println!(
        "Setting x coord: {}, ({:?})",
        coord,
        GLOBAL_POINT.lock().expect("point lock")
    );
    (*GLOBAL_POINT.lock().expect("Unwrap global point lock")).x = coord;
}

fn set_y_coord(coord: f32) {
    println!(
        "Setting y coord: {}, ({:?})",
        coord,
        GLOBAL_POINT.lock().expect("point lock")
    );
    (*GLOBAL_POINT.lock().expect("Unwrap global point lock")).y = coord;
}

fn get_x_coord() -> f32 {
    println!(
        "Getting x coord: ({:?})",
        GLOBAL_POINT.lock().expect("point lock")
    );
    (*GLOBAL_POINT.lock().expect("Unwrap global point lock")).x
}

fn get_y_coord() -> f32 {
    println!(
        "Getting y coord: ({:?})",
        GLOBAL_POINT.lock().expect("point lock")
    );
    (*GLOBAL_POINT.lock().expect("Unwrap global point lock")).y
}

async fn get_file_contents(path: String) -> Vec<u8> {
    // TODO: no path checking, module can read arbitrary file!
    let path = format!("files/{path}");

    tokio::fs::read(path)
        .await
        .expect("What could possibly go wrong")
}
