use fp_protocol::*;
use std::fmt::Write;

#[fp_export_impl(fp_protocol)]
fn add_two_u32(input: u32) -> u32 {
    add_one_u32(add_one_u32(input))
}

#[fp_export_impl(fp_protocol)]
fn move_point(point: Vector2f) -> Vector2f {
    let x = get_x_coord();
    let y = get_y_coord();

    set_x_coord(point.x);
    set_y_coord(point.y);

    Vector2f {
        x: point.x + x,
        y: point.y + y,
    }
}

#[fp_export_impl(fp_protocol)]
async fn get_data_from_files(count: u32) -> String {
    // TODO: use join_all with the futures crate
    let mut accu = String::new();

    for index in 0..count {
        write!(&mut accu, "FILE {index}:\n\n").expect("Writing to String cannot go wrong");
        let bytes = get_file_contents(format!("content{index}.txt")).await;
        accu.push_str(&String::from_utf8_lossy(&bytes));
        writeln!(&mut accu, "\n\n").expect("Writing to String cannot go wrong");
    }

    accu
}
