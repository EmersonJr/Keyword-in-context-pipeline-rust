// defining the functions to the pipeline
//
// stopWordsDefining() -> HashMap of stop words and a Boolean value -> check
// genWordsAndText() -> hashmap of words and text (String) taken from the file,
// using the stop words -> check
// genStringAns() -> sorting the hashmap of words and text -> check
// sortAndPrinting() -> sorting keys of the hashmap and printing them -> check

mod defStopWords;
mod genStringAns;
mod genWordsAndText;
mod sortAndPrinting;

use defStopWords::defStopWords::stopWordsDefining;
use genStringAns::genStringAns::genStringAns;
use genWordsAndText::genWordsAndText::genWordsAndText;
use sortAndPrinting::sortAndPrinting::sortAndPrinting;

fn main() {
    match sortAndPrinting(
        genStringAns(genWordsAndText(stopWordsDefining("./source/defStopWords.txt").unwrap(), "./source/texts.txt").unwrap(), "./source/texts.txt").unwrap(),
    ) {
        Ok(stopWords) => stopWords,
        Err(s) => panic!("{}", s),
    };
}

#[cfg(test)]
mod tests {

    use std::fs::OpenOptions;
    use std::io::Write;
    use std::collections::{HashMap, VecDeque};
    use super::defStopWords::defStopWords::stopWordsDefining;
    use super::genStringAns::genStringAns::genStringAns;
    use super::genWordsAndText::genWordsAndText::genWordsAndText;
    use super::sortAndPrinting::sortAndPrinting::sortAndPrinting;
    use super::genWordsAndText::WordContext;

    #[test]
    fn check_rand_stop_words() {

        let path = "./src/source/stopwords.txt";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path).unwrap();

        file.set_len(0);

        let stopWordsSet = vec!["a", "The", "Or", "Are", "Myself"];

        for stopWord in &stopWordsSet {
            file.write((*stopWord).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let stopWordsDefined = stopWordsDefining(path).unwrap();

        for stopWord in stopWordsSet {

            assert_eq!(stopWordsDefined.get(&String::from(stopWord).to_lowercase()), Some(&1));
        }
    }

    #[test]
    fn working_gen_words_and_text_all_txt() {

        let path = "./src/source/texts.txt";

        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path).unwrap();

        file.set_len(0);

        let texts_set = vec!["The quick brown fox", "A brown cat sat", "The cat is brown"];
        let right_texts = vec![
            "quick brown fox The",
            "brown fox The quick",
            "fox The quick brown",
            "brown cat sat A",
            "cat sat A brown",
            "cat is brown The",
            "brown The cat is"
        ];

        for txti in &texts_set {
            file.write((*txti).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let stop_words_defined = HashMap::from([
            (String::from("a"), 1),
            (String::from("the"), 1),
            (String::from("is"), 1),
            (String::from("sat"), 1)
        ]);

        let word_contexts = genWordsAndText(stop_words_defined, path).unwrap();

        let mut idx = 0;

        for word_cont in &word_contexts {

            let mut word_cont_str = String::from("");
            let mut i = 0;

            for palavra in &word_cont.phrase {

                if i == 0 {

                    word_cont_str = format!("{}", palavra);
                    i += 1;
                    continue;
                }

                word_cont_str = format!("{} {}", word_cont_str, palavra);
            }

            assert_eq!(word_cont_str.as_str(), right_texts[idx]);
            idx += 1;
        }
    }

    #[test]
    fn working_gen_words_and_text_lil_txt() {

        let path = "./src/source/texts1.txt";

        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open( &path).unwrap();

        file.set_len(0);

        let texts_set = vec!["2", "The quick brown fox", "A brown cat sat", "The cat is brown"];
        let right_texts = vec![
            "quick brown",
            "brown fox",
            "fox The",
            "brown cat",
            "cat sat",
            "cat is",
            "brown The"
        ];

        for txti in &texts_set {
            file.write((*txti).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let stop_words_defined = HashMap::from([
            (String::from("a"), 1),
            (String::from("the"), 1),
            (String::from("is"), 1),
            (String::from("sat"), 1)
        ]);

        let word_contexts = genWordsAndText(stop_words_defined, path).unwrap();

        let mut idx = 0;

        for word_cont in &word_contexts {

            let mut word_cont_str = String::from("");
            let mut i = 0;

            for palavra in &word_cont.phrase {

                if i == 0 {

                    word_cont_str = format!("{}", palavra);
                    i += 1;
                    continue;
                }

                word_cont_str = format!("{} {}", word_cont_str, palavra);
            }

            assert_eq!(word_cont_str.as_str(), right_texts[idx]);
            idx += 1;
        }
    }
}
