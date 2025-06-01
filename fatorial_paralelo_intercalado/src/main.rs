extern crate num;
use num::BigUint;
use num::One;
use std::time::Instant;
use std::thread;

fn fatorial_parcial(thread_id: u64, total_threads: u64, max_num: u64) -> BigUint {
    let mut result = BigUint::one();
    let mut num = thread_id;
    while num <= max_num {
        result *= num;
        num += total_threads;
    }
    result
}

fn main() {
    let fat = 1000000; 
    let num_threads = 1000;
    let mut handles = Vec::new();
    let start = Instant::now();

    for thread_id in 1..=num_threads {
        let handle = thread::spawn(move || {
            fatorial_parcial(thread_id, num_threads, fat)
        });
        handles.push(handle);
    }

    let mut resultado_final = BigUint::one();
    for handle in handles {
        resultado_final *= handle.join().unwrap();
    }

    println!("{}! = {}",fat, resultado_final);
    println!("Tempo de execução: {:?}", start.elapsed());
}