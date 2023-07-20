use std::sync::Mutex;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
});

struct Plugin;

static GLOBAL_VALUE: Mutex<i32> = Mutex::new(0);

impl MyWorld for Plugin {
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

    fn sqrt(num: f32) -> Result<f32, String> {
        match num {
            num if num >= 0.0 => Ok(num.sqrt()),
            _ => Err("Negative".into()),
        }
    }
}

impl exports::example::protocol::guest_exports::GuestExports for Plugin {
    fn run() {
        example::protocol::host_imports::print_line("Hello, world!");
        example::protocol::host_imports::print_line("Hello, again!");
    }
}

impl exports::inline_exports::InlineExports for Plugin {
    fn add_three(num: i32) -> i32 {
        let num = inline_imports::add_one(num);
        let num = inline_imports::add_one(num);
        let num = inline_imports::add_one(num);
        num
    }
}

export_my_world!(Plugin);
