// defining the functions to the pipeline
//
// stop_words_defining() -> HashMap of stop words and an int value
// gen_words_and_text() -> vector of a struct created to put a deque
// and a indentifier together
// gen_string_ans() -> generating the string that needs to be printed
// sort_and_printing() -> sorting the strings with the keyword in context

mod def_stop_words;
mod gen_string_ans;
mod gen_words_and_text;
mod sort_and_printing;

use def_stop_words::def_stop_words::stop_words_defining;
use gen_string_ans::gen_string_ans::gen_string_ans;
use gen_words_and_text::gen_words_and_text::gen_words_and_text;
use sort_and_printing::sort_and_printing::sort_and_printing;

fn main() {
    match sort_and_printing(
        &gen_string_ans(
            gen_words_and_text(
                stop_words_defining("./src/source/stopWords.txt").unwrap(),
                "./src/source/texts.txt",
            )
            .unwrap(),
            "./src/source/texts.txt",
        )
        .unwrap(),
    ) {
        Ok(stop_words) => stop_words,
        Err(s) => panic!("{}", s),
    };
}

#[cfg(test)]
mod tests {

    use super::def_stop_words::def_stop_words::stop_words_defining;
    use super::gen_string_ans::gen_string_ans::gen_string_ans;
    use super::gen_words_and_text::gen_words_and_text::gen_words_and_text;
    use super::sort_and_printing::sort_and_printing::sort_and_printing;
    use std::collections::HashMap;
    use std::fmt::format;
    use std::fs::OpenOptions;
    use std::io::Write;

    #[test]
    fn check_rand_stop_words() {
        let path = "./src/source/stopwords.txt";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();

        file.set_len(0).unwrap();

        let stop_words_set = vec!["a", "The", "Or", "Are", "Myself"];

        for stop_word in &stop_words_set {
            file.write((*stop_word).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let stop_words_defined = stop_words_defining(path).unwrap();

        for stop_word in stop_words_set {
            assert_eq!(
                stop_words_defined.get(&String::from(stop_word).to_lowercase()),
                Some(&1)
            );
        }
    }

    #[test]
    fn working_gen_words_and_text_all_txt() {
        let path = "./src/source/texts.txt";

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();

        file.set_len(0).unwrap();

        let texts_set = vec!["The quick brown fox", "A brown cat sat", "The cat is brown"];
        let right_texts = vec![
            "quick brown fox The",
            "brown fox The quick",
            "fox The quick brown",
            "brown cat sat A",
            "cat sat A brown",
            "cat is brown The",
            "brown The cat is",
        ];

        for txti in &texts_set {
            file.write((*txti).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let stop_words_defined = HashMap::from([
            (String::from("a"), 1),
            (String::from("the"), 1),
            (String::from("is"), 1),
            (String::from("sat"), 1),
        ]);

        let word_contexts = gen_words_and_text(stop_words_defined, path).unwrap();

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
            .open(&path)
            .unwrap();

        file.set_len(0).unwrap();

        let texts_set = vec![
            "2",
            "The quick brown fox",
            "A brown cat sat",
            "The cat is brown",
        ];
        let right_texts = vec![
            "quick brown",
            "brown fox",
            "fox The",
            "brown cat",
            "cat sat",
            "cat is",
            "brown The",
        ];

        for txti in &texts_set {
            file.write((*txti).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let stop_words_defined = HashMap::from([
            (String::from("a"), 1),
            (String::from("the"), 1),
            (String::from("is"), 1),
            (String::from("sat"), 1),
        ]);

        let word_contexts = gen_words_and_text(stop_words_defined, path).unwrap();

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
    fn working_gen_words_and_gen_stop_words() {
        let path = "./src/source/stopwords1.txt";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();

        file.set_len(0).unwrap();

        let stop_words_set = vec!["A", "ThE", "Is", "sAt"];

        for stop_word in &stop_words_set {
            file.write((*stop_word).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let path1 = "./src/source/texts2.txt";

        let mut file1 = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path1)
            .unwrap();

        file1.set_len(0).unwrap();

        let texts_set = vec![
            "2",
            "The quick brown fox",
            "A brown cat sat",
            "The cat is brown",
        ];
        let right_texts = vec![
            "quick brown",
            "brown fox",
            "fox The",
            "brown cat",
            "cat sat",
            "cat is",
            "brown The",
        ];

        for txti in &texts_set {
            file1.write((*txti).as_bytes()).unwrap();
            file1.write("\n".as_bytes()).unwrap();
        }
        let word_contexts = gen_words_and_text(stop_words_defining(path).unwrap(), path1).unwrap();

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
    fn working_gen_words_and_gen_stop_words_full_txt() {
        let path = "./src/source/stopwords2.txt";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();

        file.set_len(0).unwrap();

        let stop_words_set = vec!["A", "ThE", "Is", "sAt"];

        for stop_word in &stop_words_set {
            file.write((*stop_word).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let path1 = "./src/source/texts3.txt";

        let mut file1 = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path1)
            .unwrap();

        file1.set_len(0).unwrap();

        let texts_set = vec!["The quick brown fox", "A brown cat sat", "The cat is brown"];
        let right_texts = vec![
            "quick brown fox The",
            "brown fox The quick",
            "fox The quick brown",
            "brown cat sat A",
            "cat sat A brown",
            "cat is brown The",
            "brown The cat is",
        ];

        for txti in &texts_set {
            file1.write((*txti).as_bytes()).unwrap();
            file1.write("\n".as_bytes()).unwrap();
        }
        let word_contexts = gen_words_and_text(stop_words_defining(path).unwrap(), path1).unwrap();

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
    fn working_gen_string_ans() {
        let path = "./src/source/stopwords3.txt";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();

        file.set_len(0).unwrap();

        let stop_words_set = vec!["A", "ThE", "Is", "sAt"];

        for stop_word in &stop_words_set {
            file.write((*stop_word).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let path1 = "./src/source/texts4.txt";

        let mut file1 = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path1)
            .unwrap();

        file1.set_len(0).unwrap();

        let texts_set = vec!["The quick brown fox", "A brown cat sat", "The cat is brown"];
        let right_texts = vec![
            r##"quick brown fox The   (from "The quick brown fox")"##,
            r##"brown fox The quick   (from "The quick brown fox")"##,
            r##"fox The quick brown   (from "The quick brown fox")"##,
            r##"brown cat sat A   (from "A brown cat sat")"##,
            r##"cat sat A brown   (from "A brown cat sat")"##,
            r##"cat is brown The   (from "The cat is brown")"##,
            r##"brown The cat is   (from "The cat is brown")"##,
        ];

        for txti in &texts_set {
            file1.write((*txti).as_bytes()).unwrap();
            file1.write("\n".as_bytes()).unwrap();
        }
        let word_contexts_formated = gen_string_ans(
            gen_words_and_text(stop_words_defining(path).unwrap(), path1).unwrap(),
            path1,
        )
        .unwrap();

        let mut idx = 0;

        for resposta in word_contexts_formated {
            assert_eq!(resposta.as_str(), right_texts[idx]);

            idx += 1;
        }
    }

    #[test]
    fn working_sort_and_printing() {
        let mut to_be_sorted_texts = vec![
            r##"quick brown fox The   (from "The quick brown fox")"##.to_string(),
            r##"brown fox The quick   (from "The quick brown fox")"##.to_string(),
            r##"fox The quick brown   (from "The quick brown fox")"##.to_string(),
            r##"brown cat sat A   (from "A brown cat sat")"##.to_string(),
            r##"cat sat A brown   (from "A brown cat sat")"##.to_string(),
            r##"cat is brown The   (from "The cat is brown")"##.to_string(),
            r##"brown The cat is   (from "The cat is brown")"##.to_string(),
        ];

        let sortado = sort_and_printing(&to_be_sorted_texts).unwrap();

        to_be_sorted_texts.sort_by(|a, b| (a.to_lowercase()).cmp(&b.to_lowercase()));

        let mut pat_sorted = String::from("");

        for fras in to_be_sorted_texts {
            if pat_sorted == "" {
                pat_sorted = fras;
                continue;
            }

            pat_sorted = format!("{}\n{}", pat_sorted, fras);
        }

        assert_eq!(sortado, pat_sorted);
    }

    #[test]
    fn working_all() {
        let path = "./src/source/stopwords4.txt";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();

        file.set_len(0).unwrap();

        let stop_words_set = vec!["A", "ThE", "Is", "sAt"];

        for stop_word in &stop_words_set {
            file.write((*stop_word).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let path1 = "./src/source/texts5.txt";

        let mut file1 = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path1)
            .unwrap();

        file1.set_len(0).unwrap();

        let texts_set = vec!["The quick brown fox", "A brown cat sat", "The cat is brown"];
        let mut right_texts = vec![
            r##"quick brown fox The   (from "The quick brown fox")"##,
            r##"brown fox The quick   (from "The quick brown fox")"##,
            r##"fox The quick brown   (from "The quick brown fox")"##,
            r##"brown cat sat A   (from "A brown cat sat")"##,
            r##"cat sat A brown   (from "A brown cat sat")"##,
            r##"cat is brown The   (from "The cat is brown")"##,
            r##"brown The cat is   (from "The cat is brown")"##,
        ];

        right_texts.sort_by(|a, b| (a.to_lowercase()).cmp(&b.to_lowercase()));

        let mut pat_sorted = String::from("");

        for fras in right_texts {
            if pat_sorted == "" {
                pat_sorted = fras.to_string();
                continue;
            }

            pat_sorted = format!("{}\n{}", pat_sorted, fras);
        }

        for txti in &texts_set {
            file1.write((*txti).as_bytes()).unwrap();
            file1.write("\n".as_bytes()).unwrap();
        }

        let final_contexts = sort_and_printing(
            &gen_string_ans(
                gen_words_and_text(stop_words_defining(path).unwrap(), path1).unwrap(),
                path1,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(final_contexts, pat_sorted);
    }
}
