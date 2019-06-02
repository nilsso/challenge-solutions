#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if let Some(pos) = dna.chars().position(|c| !['A', 'C', 'G', 'T'].contains(&c)) {
            return Err(pos);
        }
        Ok(Self(dna.to_string()))
    }

    pub fn into_rna(self) -> RNA {
        let dna = self.0;
        RNA(dna
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if let Some(pos) = rna.chars().position(|c| !['A', 'C', 'G', 'U'].contains(&c)) {
            return Err(pos);
        }
        Ok(Self(rna.to_string()))
    }
}
