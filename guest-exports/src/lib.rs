wit_bindgen::generate!({
    path: "../exports.wit",
    world: "example",
    exports: {
        world: Component,
    }
});

struct Component;

impl Guest for Component {
    fn data_export(d: Data) -> Data {
        d
    }

    fn insane_data_export(d: Data, _: String, _: i32, _: Data) -> Data {
        d
    }

    fn data_list_export(d: Vec<Data>) -> Vec<Data> {
        d
    }

    fn data_deep_export(d: Vec<Vec<Data>>) -> Vec<Vec<Data>> {
        d
    }

    fn s16_export(d: Vec<i16>) -> Vec<i16> {
        d
    }

    fn u32_export(d: u32) -> u32 {
        d
    }

    fn push_s32s(mut numbers: Vec<i32>, a: i32, b: i32) -> Vec<i32> {
        numbers.push(a);
        numbers.push(b);
        numbers
    }

    fn push_u32s(mut numbers: Vec<u32>, a: u32, b: u32) -> Vec<u32> {
        numbers.push(a);
        numbers.push(b);
        numbers
    }

    fn voider() {}

    fn pairs() -> (i32, i32) {
        (0, 1)
    }

    fn small_pairs() -> (i16, i16) {
        (0, 1)
    }

    fn int_to_string(i: i32) -> String {
        i.to_string()
    }

    fn big_int(i: i64) -> i64 {
        i
    }

    fn get_ab() -> (Ab, i16) {
        todo!()
    }

    fn get_ab_list() -> Vec<(Ab, i16)> {
        todo!()
    }

    fn get_abc_list() -> Vec<Abc> {
        todo!()
    }
}
