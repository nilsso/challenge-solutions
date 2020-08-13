pub fn map<A, B>(input: Vec<A>, mut f: impl FnMut(A) -> B) -> Vec<B> {
    let mut res = Vec::with_capacity(input.len());
    for v in input {
        res.push(f(v));
    }
    res
}
