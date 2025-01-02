pub mod genStringAns {

    use crate::genWordsAndText::WordContext;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    pub fn genStringAns(contextWords: Vec<WordContext>) -> Result<Vec<String>, io::Error> {
        let mut frases = Vec::new();
        let mut originFrases = Vec::new();

        let file = File::open("./source/texts.txt")?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let linha = String::from(line.unwrap().trim());

            originFrases.push(linha);
        }

        for context in contextWords {
            let mut frase = String::from("");

            let mut aux = 0;

            for palavra in context.phrase {
                if aux == 0 {
                    frase = format!("{}", palavra);
                    aux += 1;
                    continue;
                }

                frase = format!("{} {}", frase, palavra);
            }

            frase = format!(
                r##"{}   (from "{}")"##,
                frase, originFrases[context.wher as usize]
            );

            frases.push(frase);
        }

        Ok(frases)
    }
}
