pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;

    let line = |i: usize| {
        let mut line: String = " ".repeat(2 * n + 1);
        unsafe {
            let bytes = line.as_bytes_mut();
            bytes[n - i] = i as u8 + b'A';
            bytes[n + i] = i as u8 + b'A';
        }
        line
    };

    (0..=n).chain((0..n).rev()).map(line).collect()
}
