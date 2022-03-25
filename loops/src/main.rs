fn main() {
    // Em Rust temos 3 tipos de Loops
    // FOR, While e Loop

    // Loop FOR
    for i in 0..10 {
        println!(" {}", i); // Loop que vai do 0 ao 9 concatenando 1 ao valor
    }

    // Loop While
    // Ela se repete enquando a condição for verdadeira
    let mut i = 0;
    while i <= 10 {
        println!(" {}", i);
        i += 1; // Ela não concatena 1 ao valor por padrão, então criamos um contador nela.
    }

    // Loop 
    // É um loop que se repete infinitamente
    // loop {
    //     println!("Oi");
    // }



    // Podemos para um Loop quando quisermos, usando a palavra "break".
    for i in 1..100 {
        println!("{}", i);
        if i == 2{
            println!("Parou!");
            break;
        }
    }

    let mut i = 0;
    while i <= 100 {
        println!("{}", i);
        if i == 10 {
            println!("\n Parou!");
            break;
        }
        i+= 1;

    }

    i = 0;

    loop {
        println!("{}", i);
        if i == 10 {
            println!("\n Parou!");
            break;
        }
        i+= 1;
    }




    // Podemos pular uma parte do Loop usando a palavra "continue"
    // Exemplo: temos um loop de 0 a 99 onde queremos escrever no console os numeros, exceto os multiplos de 4 e 6 ao msm tempo.
    for i in 0..100 {
        if i % 4 == 0 && i % 6 == 0 {
            continue;
        }
        println!("Numero atual {}", i);
    }
}
