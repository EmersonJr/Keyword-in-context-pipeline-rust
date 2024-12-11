use std::io;

fn main() {

    let mut inp: String = String::new();

    let n = io::stdin().read_line(&mut inp).unwrap();
    let mut value = inp.trim().parse::<i32>().unwrap();

    let mut numDivPrimes = 0;
    let mut num = 2;

    while num*num <= value {

        if value % num == 0 {

            numDivPrimes+=1;
        }

        while value % num == 0 {

            value /= num;
        }

        num+=1;
    }

    if value != 1 {

        numDivPrimes += 1;
    }


    println!("{}, value é alterado", value);
    print!("{}, resposta", numDivPrimes);
}