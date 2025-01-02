use std::collections::VecDeque;

#[derive(Clone)]
pub struct WordContext {
    pub phrase: VecDeque<String>,
    pub wher: i32,
}

pub mod gen_words_and_text {

    use super::WordContext;
    use std::cmp;
    use std::collections::{HashMap, VecDeque};
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    pub fn gen_words_and_text(
        stop_words: HashMap<String, i32>,
        path: &str,
    ) -> Result<Vec<WordContext>, io::Error> {
        let mut words_and_context: Vec<WordContext> = Vec::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut cnt = 0;
        let mut taman_janela = 0;

        for line in reader.lines() {
            let mut linha = line.unwrap();

            if cnt == 0 {
                taman_janela = match linha.trim().parse::<i32>() {
                    Ok(num) => num as usize,
                    Err(_e) => 1000000000 as usize,
                };
                if taman_janela < 1000000000 {
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

                if act.phrase.len() == cmp::min(taman_janela, frase.len() / 2) {
                    let is_stop_word =
                        match stop_words.get(&act.phrase.front().unwrap().to_lowercase()) {
                            Some(x) => *x,
                            None => 0,
                        };

                    if is_stop_word == 0 {
                        words_and_context.push(act.clone());
                    }

                    act.phrase.pop_front();
                    aux += 1;
                }
            }

            cnt += 1;
        }

        Ok(words_and_context)
    }
}
