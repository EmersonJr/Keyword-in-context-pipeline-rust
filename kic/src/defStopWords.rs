pub mod defStopWords { 

    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use std::collections::HashMap;


    pub fn stopWordsDefining() -> Result<HashMap<String, i32>, io::Error> {
    
        let mut stopWords = HashMap::new();

        let fileTxt = File::open("./source/stopwords.txt")?;
        let reader = BufReader::new(fileTxt);

        for line in reader.lines() {

            let key = String::from(line.unwrap());

            stopWords.entry(key).or_insert(1);
        }
        
        Ok(stopWords)
    }
}

// The stop words are stored in the stopwords.txt file
// one per line
