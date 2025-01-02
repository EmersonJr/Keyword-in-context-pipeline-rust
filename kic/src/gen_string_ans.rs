pub mod gen_string_ans {

    use crate::gen_words_and_text::WordContext;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    pub fn gen_string_ans(
        context_words: Vec<WordContext>,
        path: &str,
    ) -> Result<Vec<String>, io::Error> {
        let mut frases = Vec::new();
        let mut origin_frases = Vec::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let linha = String::from(line.unwrap().trim());

            origin_frases.push(linha);
        }

        for context in context_words {
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
                frase, origin_frases[context.wher as usize]
            );

            frases.push(frase);
        }

        Ok(frases)
    }
}
