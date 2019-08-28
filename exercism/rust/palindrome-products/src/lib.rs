pub mod palindrome_iter;

pub type Palindrome = u64;

fn is_palindrome_product(p: u64, min: u64, max: u64) -> bool {
    for a in min..max + 1 {
        let b = p / a;
        if b >= min && b <= max && p % (a * b) == 0 {
            return true;
        }
    }
    false
}

pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    palindrome_iter::PalindromeIter::new(min * min)
        .take_while(|&p| p < max * max)
        .filter(|p| is_palindrome_product(*p, min, max))
        .collect()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    Some(*palindromes.iter().min()?)
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    Some(*palindromes.iter().max()?)
}
