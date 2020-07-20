pub fn reverse_parentheses(s: String) -> String {
    let mut lp = None;
    let mut rp = None;
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => lp = Some(i),
            ')' => {
                rp = Some(i);
                break;
            }
            _ => {}
        }
    }
    if let (Some(l), Some(r)) = (lp, rp) {
        let rev = s[(l+1)..r].chars().rev().collect::<String>();
        let l = s[..l].to_string();
        let r = s[(r+1)..].to_string();
        reverse_parentheses(format!("{}{}{}", l, rev, r).to_string())
    } else {
        s
    }
}

fn main() {
    let s = "(Hello, (world))!".to_string();
    println!("{}", reverse_parentheses(s));
}
