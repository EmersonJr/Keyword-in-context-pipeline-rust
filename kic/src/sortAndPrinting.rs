pub mod sortAndPrinting {
    use std::io;

    pub fn sortAndPrinting(frases: Vec<String>) -> Result<String, io::Error> {
        let mut fras = frases.clone();

        let retorno = String::from("");

        fras.sort_by(|a, b| (a.to_lowercase()).cmp(&b.to_lowercase()));

        for frase in fras {
            println!("{}", frase);
        }

        Ok(retorno)
    }
}
