pub mod def_stop_words {

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    pub fn stop_words_defining(path: &str) -> Result<HashMap<String, i32>, io::Error> {
        let mut stop_words = HashMap::new();
        let file_txt = File::open(path)?;
        let reader = BufReader::new(file_txt);

        for line in reader.lines() {
            let key = String::from(line.unwrap()).to_lowercase();
            stop_words.entry(key).or_insert(1);
        }

        Ok(stop_words)
    }
}
