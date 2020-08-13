use variable_length_quantity::*;

fn main() {
    let values: &[u8] = &[
        0x40,
        0x40,
        //0x81,
        //0x80,
        //0x80,
        //0x80,
        //0x80,
        //0x00,
    ];


    let res = from_bytes(values);

    println!("{:#x?}", res);
}
