use std::io;

fn main() {

    let mut inp: String = String::new();

    let n = io::stdin().read_line(&mut inp).unwrap();
    let mut value = inp.trim().parse::<i32>().unwrap();

    let mut numDivs = 0;
    let mut num = 1;

    while num*num <= value {

        if value % num == 0 {

            numDivs+=1;
        }

        num+=1;
    }

    println!("{}, {}, os valores podem ser alterados", value, num);
    print!("{}, resposta", numDivs);
}