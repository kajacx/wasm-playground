use rand::Rng;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
});

struct Plugin;

impl MyWorld for Plugin {
    fn move_point(mut point: Point) -> Point {
        rand::thread_rng().gen::<u64>();
        point = import_point(point);
        point.x += 10;
        point.y -= 20;
        point
    }

    fn say_hello() {
        print("imported print fn");
        println!("Hello from updated (yet again) wasi plugin, will it print?");

        let time = std::time::SystemTime::now();
        // time.
        // let a = time
        //     .duration_since(std::time::SystemTime::UNIX_EPOCH)
        //     .unwrap();

        // a.as_secs();
        println!("CURRENT TIME: {:?}", time);
    }

    fn read_line() -> Option<String> {
        std::io::stdin()
            .lines()
            .next()
            .map(|line| line.expect("read line"))
    }
}

export_my_world!(Plugin);
