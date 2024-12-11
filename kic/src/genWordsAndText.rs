use std::collections::VecDeque;

pub struct wordContext {

    pub first: VecDeque<String>,
    pub second: VecDeque<String>,
}

pub mod genWordsAndText {

    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use std::collections::HashMap;

    use super::wordContext;

    pub fn genWordsAndText(stopWords : HashMap<String, i32>) -> Result<HashMap<String, wordContext>, io::Error> {

        let mut wordsAndContext: HashMap<String, wordContext> = HashMap::new();

        let file = File::open("./source/texts.txt")?;
        let reader = BufReader::new(file);
        
        for line in reader.lines() {

            let mut frase = line.unwrap().split(' ').collect();

            println!("{}", frase);

            // wordsAndContext.insert(frase[0].to_string())
        }

        Ok(wordsAndContext);
    }
}