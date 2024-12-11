// defining the functions to the pipeline
// 
// stopWordsDefining() -> HashMap of stop words and a Boolean value -> check
// TODO:genWordsAndText() -> hashmap of words and text (String) taken from the file,
// using the stop words
// TODO:sortInsideHash() -> sorting the hashmap of words and text
// TODO:sortAndPrinting() -> sorting keys of the hashmap and printing them

mod defStopWords;
mod genWordsAndText;

use defStopWords::defStopWords::stopWordsDefining;

fn main() {

    let aux = match stopWordsDefining() {
        Ok(stopWords) => stopWords,
        Err(s) => panic!("{}", s),
    };

    for (key, val)  in &aux {

        println!("{}, {}", key, val);
    }

}
