//fn all_numeric(bytes: &Vec<u8>) -> bool {
//bytes.iter().all(|&b| b'0' <= b && b <= b'9')
//}

//fn luhn_check(mut bytes: Vec<u8>) -> bool {
//bytes.iter_mut().for_each(|b| *b -= b'0');

//for b in bytes.iter_mut().rev().skip(1).step_by(2) {
//*b = match b {
//0..=4 => 2 * (*b),
//5..=9 => 2 * (*b) - 9,
//_ => unreachable!(),
//};
//}

//bytes.iter().sum::<u8>() % 10 == 0
//}

pub fn is_valid(code: &str) -> bool {
    let i = code.as_bytes().iter().filter(|&b| b != &b' ').rev();
    let (mut i, mut j) = (i.clone().skip(1).step_by(2), i.step_by(2));

    let a = i.try_fold(0, |acc, b| match b {
        b'0'..=b'4' => Ok(acc + 2 * (b - b'0')),
        b'5'..=b'9' => Ok(acc + 2 * (b - b'0') - 9),
        _ => Err(()),
    });

    let b = j.try_fold(0, |acc, &b| match b {
        b'0'..=b'9' => Ok(acc + b - b'0'),
        _ => Err(()),
    });

    a.is_ok() && b.is_ok() && (a.unwrap() + b.unwrap()) % 10 == 0
}
