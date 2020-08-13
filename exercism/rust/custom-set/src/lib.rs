use std::marker::PhantomData;
use std::ops::BitOr;

type Mask = u8;

pub trait ToMaskBit {
    fn to_mask_byte(self) -> Mask;
}

impl ToMaskBit for () {
    #[rustfmt::skip]
    fn to_mask_byte(self) -> Mask { 0 }
}

impl ToMaskBit for Mask {
    #[rustfmt::skip]
    fn to_mask_byte(self) -> Mask { 1 << (self - 1) }
}

impl ToMaskBit for &Mask {
    #[rustfmt::skip]
    fn to_mask_byte(self) -> Mask { (*self).to_mask_byte() }
}

fn mask<T: Copy + ToMaskBit>(input: &[T]) -> Mask {
    input
        .iter()
        .copied()
        .map(ToMaskBit::to_mask_byte)
        .fold(0, <Mask>::bitor)
}

#[derive(Debug, PartialEq)]
pub struct CustomSet<T = Mask> {
    mask: Mask,
    t_phantom: PhantomData<T>,
}

impl<T: Copy + ToMaskBit> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        CustomSet::from(input)
    }

    pub fn contains(&self, element: &T) -> bool {
        self.mask & element.to_mask_byte() != 0
    }

    pub fn add(&mut self, element: T) {
        self.mask |= element.to_mask_byte();
    }

    pub fn is_empty(&self) -> bool {
        self.mask == 0
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.mask == self.intersection(&other).mask
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(&other).is_empty()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet::from(self.mask & other.mask)
    }

    pub fn union(&self, other: &Self) -> Self {
        CustomSet::from(self.mask | other.mask)
    }

    pub fn complement(&self) -> Self {
        CustomSet::from(Mask::MAX - self.mask)
    }

    pub fn difference(&self, other: &Self) -> Self {
        self.intersection(&other.complement())
    }
}

impl<T> From<Mask> for CustomSet<T> {
    fn from(mask: Mask) -> Self {
        Self {
            mask,
            t_phantom: PhantomData,
        }
    }
}

impl<T: Copy + ToMaskBit> From<&[T]> for CustomSet<T> {
    fn from(input: &[T]) -> Self {
        Self::from(mask(input))
    }
}

