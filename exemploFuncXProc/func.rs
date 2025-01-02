use std::io;

fn divsPrimes(x : &mut i32, y : &mut i32) -> i32 {
    
    if (*y)*(*y) > *x {
        
        return 0;
    }
    
    if *x % *y == 0 {

        return divsPrimes(x, &mut( *y + 1)) + 2;
    }
    
    return divsPrimes(x, &mut (*y+1));
}

fn main() {

    let mut inp: String = String::new();

    let n = io::stdin().read_line(&mut inp).unwrap();
    let mut value = inp.trim().parse::<i32>().unwrap();
    let mut aux = 1;

    let ans = divsPrimes(&mut value, &mut aux);

    println!("{}, {}, os valores não são alterados", value, aux);
    print!("{}, resposta", ans);
}