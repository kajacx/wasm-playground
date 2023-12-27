wit_bindgen::generate!({
    path: "../exports.wit",
    world: "example",
    exports: {
        world: Component,
    }
});

struct Component;

impl Guest for Component {
    fn export_single(e: Matrix, a: u32) -> Matrix {
        import_single(&e, a)
    }

    fn export_single_list(e: Vec<Matrix>) -> Vec<Matrix> {
        import_single_list(&e)
    }
}
