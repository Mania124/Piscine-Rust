use std::{collections::HashMap, usize};

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut hash = HashMap::with_capacity(words.len() / 2);

    words
        .iter()
        .copied()
        .for_each(|w| *hash.entry(w).or_default() += 1);

    hash
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.keys().count()
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";
//         let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
//     let frequency_count = word_frequency_counter(words.clone());
//     let distict = nb_distinct_words(&frequency_count);
//     let mut expected = HashMap::new();
//         expected.insert("this", 2);
//         expected.insert("is", 2);
//         expected.insert("a", 2);
//         expected.insert("very", 2);
//         expected.insert("basic", 3);
//         expected.insert("sentence", 1);
//         expected.insert("with", 1);
//         expected.insert("only", 1);
//         expected.insert("few", 1);
//         expected.insert("repetitions.", 1);
//         expected.insert("once", 1);
//         expected.insert("again", 1);
//         expected.insert("but", 1);
//         expected.insert("it", 1);
//         expected.insert("should", 1);
//         expected.insert("be", 1);
//         expected.insert("enough", 1);
//         expected.insert("for", 1);
//         expected.insert("tests", 1);
//         assert_eq!(frequency_count,expected );
//         assert_eq!(distict, 19);
//     }
// }
