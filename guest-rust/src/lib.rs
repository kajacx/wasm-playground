use std::sync::Mutex;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
});

struct Guest;

static GLOBAL_VALUE: Mutex<i32> = Mutex::new(0);

impl MyWorld for Guest {
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

    fn increment_ab_export(mut variants: Vec<AbVariant>) -> Vec<AbVariant> {
        variants.iter_mut().for_each(|var| {
            *var = if let AbVariant::A((a1, a2)) = var {
                AbVariant::A((*a1 + 1, *a2 + 2))
            } else {
                *var
            }
        });
        increment_bs_import(&variants)
    }

    fn increment_deep_export(variants: Vec<Vec<AbVariant>>) -> Vec<Vec<AbVariant>> {
        let refs: Vec<_> = variants.iter().map(|x| x.as_slice()).collect();
        increment_deep_import(&refs)
    }
}

impl exports::example::protocol::guest_exports::GuestExports for Guest {
    fn run() {
        example::protocol::host_imports::print_line("Hello, world!");
        example::protocol::host_imports::print_line("Hello, again!");
    }
}

impl exports::inline_exports::InlineExports for Guest {
    fn add_three(num: i32) -> i32 {
        let num = inline_imports::add_one(num);
        let num = inline_imports::add_one(num);
        let num = inline_imports::add_one(num);
        num
    }
}

// cSpell::disable
impl exports::singlewordexports::Singlewordexports for Guest {
    fn sub_three(num: i32) -> i32 {
        let num = singlewordimports::sub_one(num);
        let num = singlewordimports::sub_one(num);
        let num = singlewordimports::sub_one(num);
        num
    }
}
// cSpell::enable

export_my_world!(Guest);
