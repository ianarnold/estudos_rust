fn main() {
    println!("Hello, world!");
    um_mais_dois();
    escreve_hello_world_dez_vezes();
    retorno_implicito();
    retorno_explicito();

    let valor = 100.0;
    let icms = calcula_icms(valor);
    let iss = calcula_iss(valor);
    escreve_icms(icms);
    escreve_iss(iss);
}

// Temos funções com ou sem argumentos.

fn um_mais_dois() -> u8 {
    return 3;
}

fn escreve_hello_world_dez_vezes() {
    for _i in 0..10 {
        println!("Hello World!");
    }
}

// Temos dois modos de realizar o retorno de uma função em Rust, 
// um deles é o retorno sendo a última linha do bloco da função 
// sem a palavra return e sem o ; o segundo modo é utilizarmos 
// a palavra return propriamente dita.

fn retorno_implicito() -> bool {
    true
}

fn retorno_explicito() -> u8 {
    if 10 > 1 {
        return 200; //a palavra return encerra a função e retorna o valor
    }
    1 //retorno implicito na mesma função
}

// Funcoes com parametros
// Recebe o valor da main e retorna;
fn calcula_icms(valor: f32) -> f32 {
    valor * 0.01
}

fn calcula_iss(valor: f32) -> f32 {
    valor * 0.10
}

fn escreve_icms(icms: f32) {
    println!("Icms: {}", icms);
}

fn escreve_iss(iss: f32) { 
    println!("Iss: {}", iss);
}