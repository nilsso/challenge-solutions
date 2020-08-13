pub struct AccumulateIter<'a, T, I>
where
    T: Copy + Clone + 'a,
    I: Iterator<Item = &'a T>,
{
    iter: I,
    state: T,
    f: fn(T, T) -> T,
}

impl<'a, T, I> Iterator for AccumulateIter<'a, T, I>
where
    T: Copy + Clone + 'a,
    I: Iterator<Item = &'a T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let Self { state, iter, f } = self;
        if let Some(next) = iter.next() {
            *state = f(*state, *next);
            Some(*state)
        } else {
            None
        }
    }
}

pub trait Accumulate<'a, T>: Iterator<Item = &'a T>
where
    T: Copy + Clone + 'a,
    Self: Sized,
{
    fn accumulate(self, init: T, f: fn(T, T) -> T) -> AccumulateIter<'a, T, Self>;
}

impl<'a, T, I> Accumulate<'a, T> for I
where
    T: Copy + Clone + 'a,
    I: Iterator<Item = &'a T>,
{
    fn accumulate(self, init: T, f: fn(T, T) -> T) -> AccumulateIter<'a, T, Self>
    where
        T: Copy + Clone,
        Self: Sized,
    {
        AccumulateIter {
            iter: self,
            state: init,
            f,
        }
    }
}
