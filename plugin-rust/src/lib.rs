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
}

export_my_world!(Plugin);
