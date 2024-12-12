// defining the functions to the pipeline
// 
// stopWordsDefining() -> HashMap of stop words and a Boolean value -> check
// genWordsAndText() -> hashmap of words and text (String) taken from the file,
// using the stop words -> check
// TODO:sortInsideHash() -> sorting the hashmap of words and text
// TODO:sortAndPrinting() -> sorting keys of the hashmap and printing them

mod defStopWords;
mod genWordsAndText;

use defStopWords::defStopWords::stopWordsDefining;
use genWordsAndText::genWordsAndText::genWordsAndText;

fn main() {

    let aux = match genWordsAndText(stopWordsDefining().unwrap()) {
        Ok(stopWords) => stopWords,
        Err(s) => panic!("{}", s),
    };

    for act in aux {

        for palavra in act.phrase {

            print!("{} ", palavra);
        }

        println!();
    }
}
