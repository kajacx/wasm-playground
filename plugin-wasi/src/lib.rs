// wit_bindgen::generate!({
//     path: "../protocol.wit",
//     world: "my-world",
// });
use bindings::*;

struct Plugin;

impl MyWorld for Plugin {
    fn move_point(mut point: Point) -> Point {
        //point = import_point(point);
        point.x += 10;
        point.y -= 20;
        point
    }

    fn say_hello() {
        println!("Hello from updated wasi plugin, will it print?");
    }
}

// export_my_world!(Plugin);
export!(Plugin);
