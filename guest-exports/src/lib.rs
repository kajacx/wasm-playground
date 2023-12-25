wit_bindgen::generate!({
    path: "../exports.wit",
    world: "example",
    exports: {
        world: Component,
    }
});

struct Component;

impl Guest for Component {
    fn export_empty(e: Empty) -> Empty {
        import_empty(e)
    }

    fn export_empty_list(e: Vec<Empty>) -> Vec<Empty> {
        import_empty_list(&e)
    }

    fn export_mby_u32(e: Option<u32>) -> Option<u32> {
        import_mby_u32(e)
    }

    fn export_mby_list(e: Vec<Option<u32>>) -> Vec<Option<u32>> {
        import_mby_list(&e)
    }

    fn export_mby_string(e: Option<String>) -> Option<String> {
        import_mby_string(e.as_deref())
    }
}
