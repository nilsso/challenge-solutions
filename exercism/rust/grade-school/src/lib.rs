use std::collections::{BTreeMap, BTreeSet};

pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0.entry(grade).or_default().insert(student.into());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        Some(self.0.get(&grade)?.iter().cloned().collect())
    }
}
