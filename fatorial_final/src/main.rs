//extern crate num;
//use num::bigint::BigUint;
use num::{BigUint, One};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Instant;
use std::thread;

//Função para realizar o cálculo do fatorial de um número
fn fatorial(num : u32, ini: u32, pre_cal: BigUint) -> BigUint {
    let mut resultado = pre_cal;
    let num_mut = num;

    for i in ini..num_mut{
        resultado *= i;
    }

    resultado
}

fn dividir_trabalho(num: u32) {
    let num_threads = 1000;//Números de threads
    let mut handles = Vec::new();//Cria um vetor para salvar ponteiros para as threads 
    
    let mapa: Arc<Mutex<HashMap<u32, BigUint>>> = Arc::new(Mutex::new(HashMap::new()));//Estrutura hash, para salvar os resultados dos fatoriais

    let range_fat = num / num_threads; //Define o range em que cada thread irá trabalhar 
    
    for i in 0..num_threads{//For para inicializar cada thread
        let inicio = i * range_fat + 1;//Cálcula o início do trabalho para a thread
        let fim = if i == num_threads - 1{//Cálculo o fim do trabalho para a thread
            num
        }else{
            (i + 1) * range_fat
        };

        println!("Thread {} calcula de {} até {}", i + 1, inicio, fim);

        let mapa_clone = Arc::clone(&mapa);//Cria um clone da tabela hash para usarmos dentro da thread


        let handle = thread::spawn(move || {//Incializa thread
            for j in inicio..=fim{//For para calcular todos os fatoriais do intervalo
                let mut ind: u32 = 1;
                let mut val = BigUint::one();
                
                //bloco para dar lock e realizar a pesquisa na hash e depois desbloquear.
                {   
                    //Habilita o lock na tabela hash evitando acesso simultaneo
                    let hash = mapa_clone.lock().unwrap();
                
                    for k in (1..j).rev() {//For reverso que procura na hash se os valores de menores que j já foram calculados 
                        if let Some(valor) = hash.get(&k) {
                            val = valor.clone();
                            ind = k;
                            break;
                        }
                    }
                }
                
                            
                //Habilita o lock na tabela hash evitando acesso simultaneo
                let mut hash = mapa_clone.lock().unwrap();
                let resultado : BigUint = fatorial(j, ind, val);
                
                hash.insert(j, resultado.clone());//Salva o calculo na hash
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
    let fat: u32 = 1000000;
    let start = Instant::now();
    dividir_trabalho(fat);
    println!("Tempo de execução: {:?}", start.elapsed());
    
}
