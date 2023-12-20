wit_bindgen::generate!({
    path: "../world.wit",
    world: "example",
    exports: {
        world: Component,
    }
});

struct Component;

impl Guest for Component {
    fn data_export(d: Data) -> Data {
        data_import(&d)
    }

    fn data_list_export(d: Vec<Data>) -> Vec<Data> {
        data_list_import(&d)
    }

    fn data_deep_export(d: Vec<Vec<Data>>) -> Vec<Vec<Data>> {
        // let d: Vec<&[Data]> = d.iter().map(Vec::as_ref).collect();
        data_deep_import(&d)
    }

    fn s16_export(d: Vec<i16>) -> Vec<i16> {
        s16_import(&d)
    }
}
