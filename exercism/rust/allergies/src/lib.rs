#![allow(non_upper_case_globals)]

use bitflags::bitflags;

bitflags! {
    pub struct Allergen: u32 {
        const Eggs = 1;
        const Peanuts = 2;
        const Shellfish = 4;
        const Strawberries = 8;
        const Tomatoes = 16;
        const Chocolate = 32;
        const Pollen = 64;
        const Cats = 128;
    }
}

const ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

pub struct Allergies(Allergen);

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(Allergen::from_bits_truncate(score))
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(*allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .filter(|a| self.0.contains(**a))
            .cloned()
            .collect()
    }
}
