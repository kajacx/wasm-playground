wit_bindgen::generate!({
    path: "../exports.wit",
    world: "example",
    exports: {
        world: Component,
    }
});

struct Component;

impl Guest for Component {
    fn export_vector_round(v: Vector) -> Vector {
        import_vector_round(v)
    }

    fn export_vector_list(l: Vec<Vector>) -> Vec<Vector> {
        import_vector_list(&l)
    }

    fn export_s32(s: i32) -> i32 {
        import_s32(s)
    }

    fn export_string(s: String) -> String {
        import_string(&s)
    }

    fn export_insane(
        i: (Vector, Vector, Vector, Vector, Vector, Vector),
    ) -> (Vector, Vector, Vector, Vector, Vector, Vector) {
        import_insane(i)
    }
}
