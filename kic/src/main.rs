// defining the functions to the pipeline
// 
// stopWordsDefining() -> HashMap of stop words and a Boolean value -> check
// genWordsAndText() -> hashmap of words and text (String) taken from the file,
// using the stop words -> check
// genStringAns() -> sorting the hashmap of words and text -> check
// TODO:sortAndPrinting() -> sorting keys of the hashmap and printing them

mod defStopWords;
mod genWordsAndText;
mod genStringAns;
mod sortAndPrinting;

use defStopWords::defStopWords::stopWordsDefining;
use genWordsAndText::genWordsAndText::genWordsAndText;
use genStringAns::genStringAns::genStringAns;
use sortAndPrinting::sortAndPrinting::sortAndPrinting;

fn main() {

    match sortAndPrinting(genStringAns(genWordsAndText(stopWordsDefining().unwrap()).unwrap()).unwrap()) {
        Ok(stopWords) => stopWords,
        Err(s) => panic!("{}", s),
    };
}
