#![no_std]

pub mod ds;

pub fn m() {
    use ds::array_obj;

    let _: array_obj::Array<1024, u8> = array_obj::Array::default();
}