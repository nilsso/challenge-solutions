pub fn rotate(input: &str, key: i8) -> String {
    let mut output = input.to_owned();

    let rotate_byte = move |b, s| (b as i8 - s as i8 + key).rem_euclid(26) as u8 + s;

    unsafe {
        for b in output.as_bytes_mut() {
            *b = match b {
                b'a'..=b'z' => rotate_byte(*b, b'a'),
                b'A'..=b'Z' => rotate_byte(*b, b'A'),
                _ => *b,
            };
        }
    }

    output
}
