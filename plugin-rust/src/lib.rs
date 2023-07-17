use std::sync::Mutex;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
});

struct Plugin;

static GLOBAL_VALUE: Mutex<i32> = Mutex::new(0);

impl MyWorld for Plugin {
    fn run() {
        print("Hello, world!");
        print("Hello, again!");
    }

    fn move_point(mut point: Point) -> Point {
        point = import_point(point);
        point.x += 10;
        point.y -= 20;
        point
    }

    fn increment() -> i32 {
        let mut lock = GLOBAL_VALUE.try_lock().unwrap();
        *lock = *lock + 1;
        *lock
    }
}

export_my_world!(Plugin);
