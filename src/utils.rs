use crate::structs::{Aminoacid, Codon};
use lazy_static::lazy_static;
use rand::seq::SliceRandom;

lazy_static! {
    /// Creates a vector holding all possible aminoacids.
    pub(crate) static ref AMINOACID_TABLE: Vec<Aminoacid> = vec![
        Aminoacid::from(('-', Vec::new())),
        Aminoacid::from((
            'a',
            vec![
                Codon::from_chars(['g', 'c', 'u']),
                Codon::from_chars(['g', 'c', 'c']),
                Codon::from_chars(['g', 'c', 'a']),
                Codon::from_chars(['g', 'c', 'g']),
            ],
        )),
        Aminoacid::from(('c', vec![Codon::from_chars(['u', 'g', 'u']), Codon::from_chars(['u', 'g', 'c'])])),
        Aminoacid::from(('d', vec![Codon::from_chars(['g', 'a', 'u']), Codon::from_chars(['g', 'a', 'c'])])),
        Aminoacid::from(('e', vec![Codon::from_chars(['g', 'a', 'a']), Codon::from_chars(['g', 'a', 'g'])])),
        Aminoacid::from(('f', vec![Codon::from_chars(['u', 'u', 'u']), Codon::from_chars(['u', 'u', 'c'])])),
        Aminoacid::from((
            'g',
            vec![
                Codon::from_chars(['g', 'g', 'u']),
                Codon::from_chars(['g', 'g', 'c']),
                Codon::from_chars(['g', 'g', 'a']),
                Codon::from_chars(['g', 'g', 'g']),
            ],
        )),
        Aminoacid::from(('h', vec![Codon::from_chars(['c', 'a', 'u']), Codon::from_chars(['c', 'a', 'c'])])),
        Aminoacid::from(('i', vec![Codon::from_chars(['a', 'u', 'u']), Codon::from_chars(['a', 'u', 'c']), Codon::from_chars(['a', 'u', 'a'])])),
        Aminoacid::from(('k', vec![Codon::from_chars(['a', 'a', 'a']), Codon::from_chars(['a', 'a', 'g'])])),
        Aminoacid::from((
            'l',
            vec![
                Codon::from_chars(['u', 'u', 'a']),
                Codon::from_chars(['u', 'u', 'g']),
                Codon::from_chars(['c', 'u', 'u']),
                Codon::from_chars(['c', 'u', 'c']),
                Codon::from_chars(['c', 'u', 'a']),
                Codon::from_chars(['c', 'u', 'g']),
            ],
        )),
        Aminoacid::from(('m', vec![Codon::from_chars(['a', 'u', 'g'])])),
        Aminoacid::from((
            'n',
            vec![
                Codon::from_chars(['c', 'c', 'u']),
                Codon::from_chars(['c', 'c', 'c']),
                Codon::from_chars(['c', 'c', 'a']),
                Codon::from_chars(['c', 'c', 'g']),
            ],
        )),
        Aminoacid::from(('p', vec![Codon::from_chars(['c', 'a', 'a']), Codon::from_chars(['c', 'a', 'g'])])),
        Aminoacid::from((
            'r',
            vec![
                Codon::from_chars(['c', 'g', 'u']),
                Codon::from_chars(['c', 'g', 'c']),
                Codon::from_chars(['c', 'g', 'a']),
                Codon::from_chars(['c', 'g', 'g']),
                Codon::from_chars(['a', 'g', 'a']),
                Codon::from_chars(['a', 'g', 'g']),
            ],
        )),
        Aminoacid::from((
            's',
            vec![
                Codon::from_chars(['u', 'c', 'u']),
                Codon::from_chars(['u', 'c', 'c']),
                Codon::from_chars(['u', 'c', 'a']),
                Codon::from_chars(['u', 'c', 'g']),
                Codon::from_chars(['a', 'g', 'u']),
                Codon::from_chars(['a', 'g', 'c']),
            ],
        )),
        Aminoacid::from((
            'v',
            vec![
                Codon::from_chars(['g', 'u', 'u']),
                Codon::from_chars(['g', 'u', 'c']),
                Codon::from_chars(['g', 'u', 'a']),
                Codon::from_chars(['g', 'u', 'g']),
            ],
        )),
        Aminoacid::from(('w', vec![Codon::from_chars(['u', 'g', 'g'])])),
        Aminoacid::from(('y', vec![Codon::from_chars(['u', 'a', 'u']), Codon::from_chars(['u', 'a', 'c'])])),
        Aminoacid::from(('*', vec![Codon::from_chars(['u', 'a', 'a']), Codon::from_chars(['u', 'a', 'g']), Codon::from_chars(['u', 'g', 'a'])])),
    ];
}

/// Select a random `String` from a given `Vector`.
pub(crate) fn select_rnd_str(string_list: &Vec<String>) -> String {
    String::from(string_list.choose(&mut rand::thread_rng()).unwrap())
}
