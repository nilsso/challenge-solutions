#[derive(Debug, PartialEq)]
pub struct DNA;

#[derive(Debug, PartialEq)]
pub struct RNA;

const NUCLEOTIDES: &[char] = &['G', 'C', 'T', 'A'];

impl DNA {
    // Construct new DNA from string.
    // If string contains invalid nucleotides return index of first invalid nucleotide
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if Some(pos) = dna.chars().position(|c| NUCLEOTIDES.contains(c)) {
            return Err(pos);
        }

    }

    // Transform DNA into corresponding RNA
    pub fn into_rna(self) -> RNA {
        RNA
    }
}

impl RNA {
    // Construct new RNA from string.
    // If string contains invalid nucleotides return index of first invalid nucleotide
    pub fn new(rna: &str) -> Result<RNA, usize> {
        RNA
    }
}
