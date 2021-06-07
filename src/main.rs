#![feature(generators, generator_trait)]

use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;

type Num = u128;

use std::env::args;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

// the smallest number x: Num, s.t. x**2 >= n
fn sqrt(n: Num) -> Num {
    let mut x = (n as f64).sqrt() as u128;
    while x * x < n {
        x += 1;
    }
    x
}

fn get_k_primes(k: usize) -> Vec<Num> {
    let mut primes = vec![2];
    let mut i: Num = 3;
    while primes.len() <  k {
        if is_prime(i) {
            primes.push(i);
        }
        i += 1
    }
    primes
}

fn is_prime(n: Num) -> bool {
    if n < 4 { return n >= 2; }
    (2..=sqrt(n)).all(|i| n%i != 0)
}

// yields the numbers from [1, ..., np.prod(primes)] which are coprime to `primes`
fn coprimes(mut primes: Vec<Num>) -> Box<dyn Iterator<Item=Num> + Send> {
    let mut generator = move || {
        if primes.len() == 1 { yield 1; }
        else {
            let last = primes.pop().unwrap();
            let prod: Num = primes.iter().fold(1, |x, y| x*y);

            for co in coprimes(primes) {
                for c in (co..(last*prod + co)).step_by(prod as usize) {
                    if c % last != 0 { yield c; }
                }
            }
        }
    };

    Box::new(std::iter::from_fn(move || {
        match Pin::new(&mut generator).resume(()) {
            GeneratorState::Yielded(co) => Some(co),
            GeneratorState::Complete(()) => None,
        }
    }))
}

fn div(semi: Num, k: usize) -> Num {
    let semi_sqrt = sqrt(semi);

    let primes = get_k_primes(k);

    for &p in &primes {
        if semi % p == 0 { return p; }
    }
    
    let prod: Num = primes.iter().fold(1, |x, y| x*y);

    coprimes(primes).par_bridge().for_each(|co| {
        for x in (co..semi_sqrt).step_by(prod as usize) {
            if semi % x == 0 && x != 1 {
                println!("{} * {}", x, semi/x);
                std::process::exit(0);
            }
        }
    });

    panic!("no divisor was found")
}

fn main() {
    let semi: Num = args().nth(1).unwrap().parse().unwrap();
	let k: usize = match args().nth(2) {
		Some(x) => x.parse().unwrap(),
		None => 7,
	};
    div(semi, k);
}
