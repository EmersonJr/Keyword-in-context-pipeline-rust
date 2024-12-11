use std::io;
use std::iter;

fn crivo(x : &mut i32) -> i32 {

    let mut vis : Vec<i32> = iter::repeat(0).take((*x+1) as usize).collect();

    let mut idx = 2;
    let mut ans = 0;

    while idx <= *x {

        if vis[idx as usize] != 0 {

            idx += 1;
            continue;
        }

        let mut j = idx+idx;

        while j <= *x {

            if j == *x {

                ans += 1;
            }

            vis[j as usize] = 1;

            j += idx;
        }

        idx += 1;
    }
    ans
}

fn main() {

    let mut inp: String = String::new();

    let n = io::stdin().read_line(&mut inp).unwrap();
    let mut value = inp.trim().parse::<i32>().unwrap();

    let ans = crivo(&mut value);

    println!("{}, value é inalterado", value);
    print!("{}, resposta", ans);
}