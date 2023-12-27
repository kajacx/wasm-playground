use rand::Rng;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
    exports: {
        world: Plugin
    }
});

struct Plugin;

impl Guest for Plugin {
    fn say_hello() {
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

    fn add_three(num: u64) -> u64 {
        num + 3
    }
}
