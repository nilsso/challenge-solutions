pub fn is_valid_isbn(isbn: &str) -> bool {
    let x: Vec<u32> = isbn
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .enumerate()
        .filter_map(|(i, c)| {
            println!("{} {}", i, c);
            if i == 9 && c == 'x' {
                Some(10)
            } else {
                c.to_digit(10)
            }
        })
        .collect();
    x.len() == 10 && x.iter().zip((1..11).rev()).map(|(d, i)| i * d).sum::<u32>() % 11 == 0
}
