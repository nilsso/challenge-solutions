#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for mut v in values.iter().cloned() {
        let mut temp: Vec<u8> = Vec::new();
        loop {
            let r = v % 128;
            temp.push(r as u8 | 128);
            v /= 128;
            if v == 0 {
                break;
            }
        }
        temp[0] &= 127;
        bytes.extend(temp.into_iter().rev());
    }
    bytes
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut values: Vec<u32> = Vec::new();
    let mut temp: u32 = 0;
    for v in bytes.iter() {
        temp = temp * 128 + (v & 127) as u32;
        if v & 128 == 0 {
            values.push(temp);
            temp = 0;
        }
    }
    if temp != 0 {
        Err(Error::IncompleteNumber)
    } else {
        Ok(values)
    }
}
