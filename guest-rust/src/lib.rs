use std::sync::Mutex;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
    exports: {
        world: MyGuest,
        "example:protocol/guest-exports": MyGuest,
        "inline-exports": MyGuest,
        "singlewordexports": MyGuest,
    }
});

struct MyGuest;

static GLOBAL_VALUE: Mutex<i32> = Mutex::new(0);

impl Guest for MyGuest {
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

    fn export_flags() -> Permissions {
        Permissions::READ.union(Permissions::WRITE)
    }

    fn export_many_flags() -> ManyFlags {
        ManyFlags::F05.complement()
    }
}

impl exports::example::protocol::guest_exports::Guest for MyGuest {
    fn run() {
        example::protocol::host_imports::print_line("Hello, world!");
        example::protocol::host_imports::print_line("Hello, again!");
    }
}

impl exports::inline_exports::Guest for MyGuest {
    fn add_three(num: i32) -> i32 {
        let num = inline_imports::add_one(num);
        let num = inline_imports::add_one(num);
        let num = inline_imports::add_one(num);
        num
    }
}

// cSpell::disable
impl exports::singlewordexports::Guest for MyGuest {
    fn sub_three(num: i32) -> i32 {
        let num = singlewordimports::sub_one(num);
        let num = singlewordimports::sub_one(num);
        let num = singlewordimports::sub_one(num);
        num
    }
}
// cSpell::enable
