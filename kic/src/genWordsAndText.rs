use std::{collections::VecDeque, path::Iter};

#[derive(Clone)]
pub struct WordContext {
    pub phrase: VecDeque<String>,
    pub wher: i32,
}

pub mod genWordsAndText {

    use super::WordContext;
    use std::cmp;
    use std::collections::{HashMap, VecDeque};
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    pub fn genWordsAndText(stopWords: HashMap<String, i32>, path: &str) -> Result<Vec<WordContext>, io::Error> {
        let mut wordsAndContext: Vec<WordContext> = Vec::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut cnt = 0;
        let mut tamanJanela = 0;

        for line in reader.lines() {
            let mut linha = line.unwrap();

            if cnt == 0 {
                tamanJanela = match linha.trim().parse::<i32>(){

                    Ok(num) => num as usize,
                    Err(e) => 1000000000 as usize,
                };
                if tamanJanela < 1000000000 {
                    cnt += 1;
                    continue;
                }
            }

            linha = format!("{} {}", linha, linha);

            let frase: Vec<&str> = linha.as_str().split(' ').collect();

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
                    let isStopWord = match stopWords.get(&act.phrase.front().unwrap().to_lowercase()) {
                        Some(x) => *x,
                        None => 0,
                    };

                    if isStopWord == 0 {
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
