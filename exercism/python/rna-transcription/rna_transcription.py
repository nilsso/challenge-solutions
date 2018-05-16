import re
def to_rna(dna_strand):
    if re.search("[^GCTA]", dna_strand):
        raise ValueError("Input contains invalid nucleotide values")
    return dna_strand.translate(str.maketrans("GCTA", "CGAU"))
