extern crate svg;
use svg::node::element::path::Data;

pub fn create_0_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .close();
    return data;
}

pub fn create_1_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((25, 25))
                .close();
    return data;
}

pub fn create_2_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 25))
                .line_by((-25, 25))
                .close();
    return data;
}

pub fn create_3_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 25))
                .line_by((-25, 25))
                .close();
    return data;
}

pub fn create_4_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset + 50))
                .line_by((25, 25))
                .close();
    return data;
}

pub fn create_5_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position, y_offset + 50))
                .line_by((25, 25))
                .close();
    return data;
}

pub fn create_6_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 25))
                .line_by((-25, 25))
                .move_to((x_offset + offset_increment * position, y_offset + 50))
                .line_by((25, 25))
                .close();
    return data;
}

pub fn create_7_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 25))
                .line_by((-25, 25))
                .move_to((x_offset + offset_increment * position, y_offset + 50))
                .line_by((25, 25))
                .close();
    return data;
}

pub fn create_8_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 75))
                .line_by((-25, 25))
                .close();
    return data;
}

pub fn create_9_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 75))
                .line_by((-25, 25))
                .close();
    return data;
}

pub fn create_a_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 25))
                .line_by((-25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 75))
                .line_by((-25, 25))
                .close();
    return data;
}

pub fn create_b_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 25))
                .line_by((-25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 75))
                .line_by((-25, 25))
                .close();
    return data;
}

pub fn create_c_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset + 50))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 75))
                .line_by((-25, 25))
                .close();
    return data;
}

pub fn create_d_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position, y_offset + 50))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 75))
                .line_by((-25, 25))
                .close();
    return data;
}

pub fn create_e_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 25))
                .line_by((-25, 25))
                .move_to((x_offset + offset_increment * position, y_offset + 50))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 75))
                .line_by((-25, 25))
                .close();
    return data;
}

pub fn create_f_data(offset_increment: i32, position: i32) -> svg::node::element::path::Data {
    let x_offset = 10;
    let y_offset = 10;
    let data = Data::new()
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((0, 100))
                .move_to((x_offset + offset_increment * position, y_offset))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 25))
                .line_by((-25, 25))
                .move_to((x_offset + offset_increment * position, y_offset + 50))
                .line_by((25, 25))
                .move_to((x_offset + offset_increment * position + 25, y_offset + 75))
                .line_by((-25, 25))
                .close();
    return data;
}