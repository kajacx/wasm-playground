wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
});

struct Plugin;

impl MyWorld for Plugin {
    fn run() {
        print("Hello, world!");
        print("Hello, again!");
    }

    fn move_point(mut point: Point) -> Point {
        point = import_point(point);
        point.x += 10;
        point.y -= 2;
        point
    }
}

export_my_world!(Plugin);
