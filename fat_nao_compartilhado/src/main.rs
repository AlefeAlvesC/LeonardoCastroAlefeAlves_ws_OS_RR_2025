//extern crate num;
//use num::bigint::BigUint;
use num::{BigUint, One};
use std::time::Instant;
use std::thread;

//Função para realizar o cálculo do fatorial de um número
fn fatorial(num : BigUint) -> BigUint {
    let mut resultado = BigUint::one();
    let mut num_mut = num;

    while num_mut > BigUint::one(){
        resultado *= num_mut.clone();
        num_mut -= BigUint::one(); 
    }
    resultado
}

fn dividir_trabalho(num: u32) {
    let num_threads = 100;//Números de threads
    let mut handles = Vec::new();//Cria um vetor para salvar ponteiros para as threads 
    
    
    let range_fat = num / num_threads; //Define o range em que cada thread irá trabalhar 
    
    for i in 0..num_threads{//For para inicializar cada thread
        let inicio = i * range_fat + 1;//Cálcula o início do trabalho para a thread
        let fim = if i == num_threads - 1{//Cálculo o fim do trabalho para a thread
            num
        }else{
            (i + 1) * range_fat
        };

        println!("Thread {} calcula de {} até {}", i + 1, inicio, fim);

       
        let handle = thread::spawn(move || {
            for j in inicio..=fim{//For para calcular todos os fatoriais do intervalo
                
                let resultado : BigUint = fatorial(BigUint::from(j));
                println!("O fatorial de {} calculado pela thread {} é {}.", j, i+1, resultado);
            }
            
        });

        handles.push(handle);
    }
    
    //let mut resultado_final = BigUint::one();
    for handle in handles {
        handle.join().unwrap();
    }
    
    //println!("Fatorial de {} é {}", num, resultado_final);
    
}

fn main() {
    let fat: u32 = 100000;
    let start = Instant::now();
    dividir_trabalho(fat);
    println!("Tempo de execução: {:?}", start.elapsed());
    
}
