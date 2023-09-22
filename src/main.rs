use rayon::prelude::*;

fn getNthPotentialPerfectNum(n: u64) -> u64 {
    let mut sum: u64 = 0;
    let mut factor = 1;
    for i in 0..n {
        sum += factor;
        factor <<= 1;
    }

    factor = sum;

    for i in 0..(n - 1) {
        sum += factor;
        factor *= 2;
    }

    sum
}

fn isPerfectNum(num: u64) -> bool {
    let mut sum = 0;
    for i in 1..=(num / 2) {
        if (num % i == 0) {
            sum += i;
        }
    }
    num == sum
}
fn main() {
    (1..18)
        .into_par_iter()
        .map(getNthPotentialPerfectNum)
        .filter(|&i| isPerfectNum(i))
        .for_each(|i| println!("{i}"));
}
