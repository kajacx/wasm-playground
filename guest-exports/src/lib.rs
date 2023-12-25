wit_bindgen::generate!({
    path: "../exports.wit",
    world: "example",
    exports: {
        world: Component,
    }
});

struct Component;

impl Guest for Component {
    fn export_single(e: Single) -> Single {
        import_single(e)
    }

    fn export_single_list(e: Vec<Single>) -> Vec<Single> {
        import_single_list(&e)
    }

    fn export_result(_e: Result<Single, String>) -> Result<Single, String> {
        import_result(todo!())
    }

    fn export_result_list(e: Vec<Result<Single, String>>) -> Vec<Result<Single, String>> {
        import_result_list(&e)
    }

    fn mby(e: Option<Single>) -> Option<Single> {
        e
    }
}
