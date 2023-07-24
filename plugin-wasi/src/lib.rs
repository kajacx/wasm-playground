wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
});

struct Plugin;

impl MyWorld for Plugin {
    fn move_point(mut point: Point) -> Point {
        point = import_point(point);
        point.x += 10;
        point.y -= 20;
        point
    }

    fn say_hello() {
        print("imported print fn");
        println!("Hello from updated (yet again) wasi plugin, will it print?");
    }

    fn read_line() -> Option<String> {
        std::io::stdin()
            .lines()
            .next()
            .map(|line| line.expect("read line"))
    }
}

export_my_world!(Plugin);
