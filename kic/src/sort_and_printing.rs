pub mod sort_and_printing {
    use std::io;

    pub fn sort_and_printing(frases: Vec<String>) -> Result<String, io::Error> {
        let mut fras = frases.clone();

        let retorno = String::from("");

        fras.sort_by(|a, b| (a.to_lowercase()).cmp(&b.to_lowercase()));

        for frase in fras {
            println!("{}", frase);
        }

        Ok(retorno)
    }
}