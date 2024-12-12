use std::collections::VecDeque;
#[derive(Clone)]  
pub struct WordContext {

    pub phrase: VecDeque<String>,
    pub wher: i32,
}

pub mod genWordsAndText {
    
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use std::collections::{HashMap, VecDeque};
    use std::cmp;
    use super::WordContext;

    pub fn genWordsAndText(stopWords : HashMap<String, i32>) -> Result<Vec<WordContext>, io::Error> {

        let mut wordsAndContext: Vec<WordContext> = Vec::new();

        let file = File::open("./source/texts.txt")?;
        let reader = BufReader::new(file);

        let mut cnt = 0;
        let mut tamanJanela = 0;

        for line in reader.lines() {

            let mut linha = line.unwrap();

            if cnt == 0 {
                
                tamanJanela = linha.trim().parse::<i32>().unwrap() as usize;
                cnt += 1;
                continue;
            }

            linha = format!("{} {}", linha, linha);

            let mut frase : Vec<&str> = linha.as_str().split(' ').collect();

            let mut act = WordContext {
                phrase: VecDeque::new(),
                wher: cnt,
            };
            
            let mut aux = 0;

            for palavra in &frase {

                if aux >= (frase.len() / 2) {

                    break;
                }

                act.phrase.push_back(String::from(*palavra));

                if act.phrase.len() == cmp::min(tamanJanela, frase.len() / 2) {

                    let num = match stopWords.get(act.phrase.front().unwrap()) {
                        Some(x)  => *x,
                        None => 0,
                    };
    
                    if num == 0 {
    
                        wordsAndContext.push(act.clone());
                    }

                    act.phrase.pop_front();
                    aux += 1;
                }
            }

            cnt += 1;
        }

        Ok(wordsAndContext)
    }
}