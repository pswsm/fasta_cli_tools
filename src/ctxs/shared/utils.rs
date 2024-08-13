use lazy_static::lazy_static;
use rand::seq::SliceRandom;

use crate::ctxs::protein::aminoacid::domain::aminoacid::Aminoacid;

lazy_static! {
    /// Creates a vector holding all possible aminoacids.
    pub(crate) static ref AMINOACID_TABLE: Vec<Aminoacid> = vec![
        Aminoacid::from(('-', Vec::new())),
        Aminoacid::from((
            'a',
            vec![
                ['g', 'c', 'u'],
                ['g', 'c', 'c'],
                ['g', 'c', 'a'],
                ['g', 'c', 'g'],
            ],
        )),
        Aminoacid::from(('c', vec![['u', 'g', 'u'], ['u', 'g', 'c']])),
        Aminoacid::from(('d', vec![['g', 'a', 'u'], ['g', 'a', 'c']])),
        Aminoacid::from(('e', vec![['g', 'a', 'a'], ['g', 'a', 'g']])),
        Aminoacid::from(('f', vec![['u', 'u', 'u'], ['u', 'u', 'c']])),
        Aminoacid::from((
            'g',
            vec![
                ['g', 'g', 'u'],
                ['g', 'g', 'c'],
                ['g', 'g', 'a'],
                ['g', 'g', 'g'],
            ],
        )),
        Aminoacid::from(('h', vec![['c', 'a', 'u'], ['c', 'a', 'c']])),
        Aminoacid::from(('i', vec![['a', 'u', 'u'], ['a', 'u', 'c'], ['a', 'u', 'a']])),
        Aminoacid::from(('k', vec![['a', 'a', 'a'], ['a', 'a', 'g']])),
        Aminoacid::from((
            'l',
            vec![
                ['u', 'u', 'a'],
                ['u', 'u', 'g'],
                ['c', 'u', 'u'],
                ['c', 'u', 'c'],
                ['c', 'u', 'a'],
                ['c', 'u', 'g'],
            ],
        )),
        Aminoacid::from(('m', vec![['a', 'u', 'g']])),
        Aminoacid::from((
            'n',
            vec![
                ['c', 'c', 'u'],
                ['c', 'c', 'c'],
                ['c', 'c', 'a'],
                ['c', 'c', 'g'],
            ],
        )),
        Aminoacid::from(('p', vec![['c', 'a', 'a'], ['c', 'a', 'g']])),
        Aminoacid::from((
            'r',
            vec![
                ['c', 'g', 'u'],
                ['c', 'g', 'c'],
                ['c', 'g', 'a'],
                ['c', 'g', 'g'],
                ['a', 'g', 'a'],
                ['a', 'g', 'g'],
            ],
        )),
        Aminoacid::from((
            's',
            vec![
                ['u', 'c', 'u'],
                ['u', 'c', 'c'],
                ['u', 'c', 'a'],
                ['u', 'c', 'g'],
                ['a', 'g', 'u'],
                ['a', 'g', 'c'],
            ],
        )),
        Aminoacid::from((
            'v',
            vec![
                ['g', 'u', 'u'],
                ['g', 'u', 'c'],
                ['g', 'u', 'a'],
                ['g', 'u', 'g'],
            ],
        )),
        Aminoacid::from(('w', vec![['u', 'g', 'g']])),
        Aminoacid::from(('y', vec![['u', 'a', 'u'], ['u', 'a', 'c']])),
        Aminoacid::from(('*', vec![['u', 'a', 'a'], ['u', 'a', 'g'], ['u', 'g', 'a']])),
    ];
}

/// Select a random `String` from a given `Vector`.
pub(crate) fn select_rnd_str(string_list: &Vec<String>) -> String {
    String::from(string_list.choose(&mut rand::thread_rng()).unwrap())
}

/// Select a random `String` from a given `Vector`.
pub(crate) fn select_rnd_char(char_list: [char; 4]) -> char {
    char_list
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned()
}
