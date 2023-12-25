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

    fn export_partial(e: Partial) -> Partial {
        import_partial(&e)
    }

    fn export_partial_list(e: Vec<Partial>) -> Vec<Partial> {
        import_partial_list(&e)
    }
}
