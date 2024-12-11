pub mod genWordsAndText {

    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use std::collections::HashMap;

    pub fn genWordsAndText(stopWords : HashMap<String, i32>) -> Result<HashMap<,>, io::Error> {

        let mut wordsAndContext: HashMap<&str, Vec<&str>> = HashMap::new();

        let file = File::open("./source/texts.txt")?;
        let reader = BufReader::new(file);

        wordsAndContext.insert("AAAA", vec!["AAA", "BBBB", "CCC"]);
        
        for line in reader.lines() {

            let mut frase = line.unwrap().split(' ').collect();

            println!("{}", frase);

            // wordsAndContext.insert(frase[0].to_string())
        }

        Ok(wordsAndContext);
    }
}