<h1>
  <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-original.svg" alt="Símbolo Rust" width="30" style="vertical-align: middle;">
  Fatorial usando Multithreading
</h1>

**Repositório referente ao seminário de SO com tema Fatorial usando multithreading**

---

### Sumário

- [Sumário](#sumário)
- [Código do Fatorial usando Multithreading em Rust](#código-do-fatorial-usando-multithreading-em-rust)
- [Rede de Petri](#rede-de-petri)
- [Slide da Apresentação](#slide-da-apresentação)
- [Referências](#referências)
- [Ferramentas e linguagens utilizadas](#ferramentas-e-linguagens-utilizadas)

**Integrantes:** [Leonardo Castro](https://github.com/thetwelvedev) e [Álefe Alves](https://github.com/AlefeAlvesC).

**Descrição:** O projeto a ser desenvolvido consiste em calcular o fatorial de todos os números até 1.000.000. Utilizando threads para cada faixa de 1000 valores, criaremos uma thread e dispararemos o processo para cada uma delas. O fatorial de um número inteiro e positivo “n”, representado por “n!”, é obtido a partir da multiplicação de todos os seus antecessores até o número um, cuja expressão genérica é: n! = n × (n – 1) × (n – 2) × ... × 2 × 1

### Código do Fatorial usando Multithreading em Rust
```rust
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
```

### Rede de Petri
![Rede de petri](/rede_de_petri/faotrial.png)

### Slide da Apresentação
[Acesse aqui](/slide/SO%20Fatorial%20usando%20multithreading.pdf)

### Referências
- PROGRAMMING IDIOMS. Recursive factorial (simple). Disponível em: https://programming-idioms.org/idiom/31/recursive-factorial-simple/450/rust. Acesso em: 31 maio 2025.

- USERS RUST-LANG. Parallel product for factorial – surprised by the results. 14 maio 2019. Disponível em: https://users.rust-lang.org/t/parallel-product-for-factorial-surprised-by-the-results/30776/14. Acesso em: 31 maio 2025.

- DANDYVICA. Using threads on Rust – Part 3. Dev.to, 17 nov. 2020. Disponível em: https://dev.to/dandyvica/using-threads-on-rust-part-3-2bpf. Acesso em: 31 maio 2025.

---

### Ferramentas e linguagens utilizadas
<div>
  <img src="https://img.shields.io/badge/-VS%20Code-007ACC?logo=visual-studio-code&logoColor=white&style=flat" alt="VS Code">
  <img src="https://img.shields.io/badge/-Rust-000000?logo=rust&logoColor=white&style=flat" alt="Rust">
</div>
