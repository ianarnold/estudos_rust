fn main() {
    println!("
    +------+----------+-----------------------------------------+------------------------------------------+-----------+
    | Tipo | Tamanho  |              Valor Máximo               |               Valor Mínimo               |           |
    +------+----------+-----------------------------------------+------------------------------------------+-----------+
    | i8   | 1 byte   | 127                                     | -128                                     | Numérico  |
    | u8   | 1 byte   | 255                                     | 0                                        | Numérico  |
    | i16  | 2 bytes  | 32767                                   | -32768                                   | Numérico  |
    | u16  | 2 bytes  | 65535                                   | 0                                        | Numérico  |
    | i32  | 4 bytes  | 2147483647                              | -2147483648                              | Numérico  |
    | u32  | 4 bytes  | 4294967295                              | 0                                        | Numérico  |
    | i64  | 8 bytes  | 9223372036854775807                     | -9223372036854775808                     | Numérico  |
    | u64  | 8 bytes  | 18446744073709551615                    | 0                                        | Numérico  |
    | i128 | 16 bytes | 170141183460469231731687303715884105727 | -170141183460469231731687303715884105728 | Numérico  |
    | u128 | 16 bytes | 340282366920938463463374607431768211455 | 0                                        | Numérico  |
    | f32  | 4 bytes  | 340282350...                            | -340282350...                            | Numérico  |
    | f64  | 8 bytes  | 1797693134862315700...                  | -1797693134862315700...                  | Numérico  |
    | bool | 1 byte   | true                                    | false                                    | booleano  |
    | char | depende  | depende                                 | depende                                  | character |
    +------+----------+-----------------------------------------+------------------------------------------+-----------+
    Temos também o tipo usize que vai dependender da arquitetura do sistema operacional.");

    
    // Pode-se definir o tipo da váriavel ou não, logo após sua declaração.
    let idade: u8 = 18; // Variável idade do tipo u8, 1 byte.
    let ano_nascimento = 2003; // Inferencia de tipo.
    println!("\n \n Idade: {}, ano nascimento: {}", idade, ano_nascimento); // Printando os valores das variáveis na tela.


    // Podemos tambem inferir o tipo
    let idade_atual: u8 = 22;
    let ano_nascimento = 1999_u16; // O padrão para inteiros é i32, mas é um desperdício de bytes utilizar ele quando podemos definir um valor menor.

    println!("\n \n Idade atual {}, ano de nascimento: {}", idade_atual, ano_nascimento);


    // Váriavel usando "mut", isso define que ela poderá ter seu valor alterado no decorrer do projeto. 
    // Diferente de uma sem isso, que tem um valor definido na criação e não poderá ser alterado.
    let mut verificado:bool = true; // Variavel do tipo booleano, TRUE or FALSE.
    println!("\n \n Verificado: {}", verificado);
    verificado = false;
    println!(" Verificado: {}", verificado);


}
