wit_bindgen::generate!("my-world");

struct Plugin;

impl MyWorld for Plugin {
    fn run() {
        print("Hello, world!");
        print("Hello, again!");
    }
}

export_my_world!(Plugin);
