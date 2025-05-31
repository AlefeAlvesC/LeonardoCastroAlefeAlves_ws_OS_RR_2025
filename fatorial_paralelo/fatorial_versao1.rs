use num::BigUint;
//extern crate num;
//use num::bigint::BigUint;
use num::One;
use std::time::Instant;
use std::thread;

fn fatorial_parcial(inicio: u64, fim: u64) -> BigUint {
    let mut result = BigUint::one();
    for i in inicio..=fim {
        result *= i;
    }
    result
}

fn main() {
    let fat = 1000000;
    let num_threads = 1000;//Levando em torno de 30 minutos, dependendo do pc

    let mut handles = Vec::new();//Cria um vetor para as salvar os resultados
    
    let range_fat = fat/num_threads; //Define o range 
    let start = Instant::now();
    
    for i in 0..num_threads{
        let inicio = i * range_fat + 1;
        let fim = if i == num_threads - 1{
            fat
        }else{
            (i + 1) * range_fat
        };

        println!("Thread {} calcula de {} até {}", i + 1, inicio, fim);

        let handle = thread::spawn(move || {
            fatorial_parcial(inicio, fim)
        });

        handles.push(handle);
    }
    
    let mut resultado_final = BigUint::one();
    for handle in handles {
        resultado_final *= handle.join().unwrap();
    }
    
    println!("Fatorial de {} é {}", fat, resultado_final);
    println!("Tempo de execução: {:?}", start.elapsed());
    
}
