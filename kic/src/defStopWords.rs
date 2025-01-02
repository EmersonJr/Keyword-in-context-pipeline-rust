pub mod defStopWords {

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    pub fn stopWordsDefining() -> Result<HashMap<String, i32>, io::Error> {
        let mut stopWords = HashMap::new();
        let fileTxt = File::open("./src/source/stopwords.txt")?;
        let reader = BufReader::new(fileTxt);

        for line in reader.lines() {
            let key = String::from(line.unwrap()).to_lowercase();
            stopWords.entry(key).or_insert(1);
        }

        Ok(stopWords)
    }
}

#[cfg(test)]
mod tests {

    use std::fs::OpenOptions;
    use std::io::Write;

    use super::defStopWords::stopWordsDefining;

    #[test]
    fn check_rand_stop_words() {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open( "./src/source/stopwords.txt").unwrap();

        file.set_len(0);

        let stopWordsSet = vec!["a", "The", "Or", "Are", "Myself"];

        for stopWord in &stopWordsSet {
            file.write((*stopWord).as_bytes()).unwrap();
            file.write("\n".as_bytes()).unwrap();
        }

        let stopWordsDefined = stopWordsDefining().unwrap();

        for stopWord in stopWordsSet {

            assert_eq!(stopWordsDefined.get(&String::from(stopWord).to_lowercase()), Some(&1));
        }
    }
}
