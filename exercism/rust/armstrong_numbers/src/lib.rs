pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10))
        .map(|o| o.unwrap())
        .collect();
    let n = digits.len() as u32;
    num == digits.iter().map(|x| x.pow(n)).sum()
}
