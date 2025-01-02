pub mod defStopWords {

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    pub fn stopWordsDefining(path : &str) -> Result<HashMap<String, i32>, io::Error> {
        let mut stopWords = HashMap::new();
        let fileTxt = File::open(path)?;
        let reader = BufReader::new(fileTxt);

        for line in reader.lines() {
            let key = String::from(line.unwrap()).to_lowercase();
            stopWords.entry(key).or_insert(1);
        }

        Ok(stopWords)
    }
}