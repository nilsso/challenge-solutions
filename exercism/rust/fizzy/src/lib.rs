pub struct Matcher<T> {
    matcher: fn(T) -> bool,
    sub: String,
}

impl<T> Matcher<T> {
    pub fn new<S>(matcher: fn(T) -> bool, sub: S) -> Matcher<T>
    where
        S: ToString,
    {
        let sub = sub.to_string();
        Self { matcher, sub }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: Copy + ToString,
{
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |t| {
            let res = self
                .matchers
                .iter()
                .filter(|m| (m.matcher)(t))
                .fold(String::new(), |res, m| res + &m.sub);
            if res.is_empty() {
                t.to_string()
            } else {
                res
            }
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Copy + ToString + From<u8> + PartialEq + std::ops::Rem<Output = T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|x: T| x % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|x: T| x % 5.into() == 0.into(), "buzz"))
}
