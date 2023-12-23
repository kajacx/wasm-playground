wit_bindgen::generate!({
    path: "../exports.wit",
    world: "example",
    exports: {
        world: Component,
    }
});

struct Component;

impl Guest for Component {
    fn xy_export(_d: Xy) -> Xy {
        todo!()
    }

    fn xy_export_tuple(_d: (Xy, Xy)) -> (Xy, Xy) {
        todo!()
    }

    fn xy_export_pair(_d: Xy, _e: Xy) -> (Xy, Xy) {
        todo!()
    }

    fn xy_export_list(_d: Vec<Xy>) -> Vec<Xy> {
        todo!()
    }

    fn xyz_export(_d: Xyz) -> Xyz {
        todo!()
    }

    fn xyz_export_tuple(_d: (Xyz, Xyz)) -> (Xyz, Xyz) {
        todo!()
    }

    fn xyz_export_pair(_d: Xyz, _e: Xyz) -> (Xyz, Xyz) {
        todo!()
    }

    fn xyz_export_list(_d: Vec<Xyz>) -> Vec<Xyz> {
        todo!()
    }

    fn void_args_s32() -> i32 {
        todo!()
    }

    fn void_ret_s32(_d: i32) {
        todo!()
    }

    fn void_both() {
        todo!()
    }

    fn points(_d: Vec<Point16>) -> Vec<Point16> {
        todo!()
    }

    fn points_more(_d: Vec<PointMore>) -> Vec<PointMore> {
        todo!()
    }

    fn points_final(_d: Vec<PointFinal>) -> Vec<PointFinal> {
        todo!()
    }

    fn player_look_at(_p: Player, _l: Player) -> Player {
        todo!()
    }

    fn char_round(_c: char) -> char {
        todo!()
    }

    fn char_list(_c: Vec<char>) -> Vec<char> {
        todo!()
    }

    fn bool_round(_c: bool) -> bool {
        todo!()
    }

    fn bool_list(_c: Vec<bool>) -> Vec<bool> {
        let _ = import_s32(5);
        import_vector_arg(Vector { x: 1, y: 2, z: 3 });
        let _ = import_vector_res();
        todo!()
    }
}
