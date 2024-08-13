use crate::ctxs::fasta::domain::fasta::C_DNA_BASES;

struct DnaGenerator {
    generator: [char; 4],
}

impl DnaGenerator {
    fn new(generates: [char; 4]) -> Self {
        Self {
            generator: generates,
        }
    }
}

fn generate() {
    let dna_generator: DnaGenerator = DnaGenerator::new(C_DNA_BASES);
}
