fn main() {
    println!("  OPERADORES MATEMÁTICOS
    +----------------------+---------+------------------------------+
    |         Tipo         | Simbolo |             Ação             |
    +----------------------+---------+------------------------------+
    | Soma                 | +       | Soma dois valores            |
    | Subtração            | -       | Subtrai dois valores         |
    | Multiplicação        | *       | Multiplica dois valores      |
    | Divisão              | /       | Divide dois valores          |
    | Mod                  | %       | Resto de divisão             |
    | Soma atribui         | +=      | Soma e atribui o valor       |
    | Subtrai e atribui    | -=      | Subtrai e atribui o valor    |
    | Multiplica e atribui | *=      | Multiplica e atribui o valor |
    | Divide e atribui     | /=      | Divide e atribui o valor     |
    +----------------------+---------+------------------------------+");

    println!("Operadores matemáticos");
    let mut n = 10 - 1; //n = 9
    n = 1 + 2; //n = 3
    n = 10 * 2; //n = 20
    n = 10 / 2; //n = 5
    /*
    Agora temos o valor de n = a 5, iremos realizar operações de atribuições com base neste valor.
    */
    n += 1; //n = 6
    n -= 2; //n = 4
    n *= 3; //n = 12
    n /= 4; //n = 3

    println!("Valor de N: {}", n);

    println!("\n \n 
    +-------------+---------+-----------------------------------------------+
    |    Tipo     | Simbolo |                     Ação                      |
    +-------------+---------+-----------------------------------------------+
    | Igual       | ==      | Compara se dois valores são iguais            |
    | Diferente   | !=      | Verifica se dois valores são diferentes       |
    | Maior       | >       | Verifica se um valor é maior que outro        |
    | Menor       | <       | Verifica se um valor é menor que outro        |
    | Maior igual | >=      | Verifica se um valor é maior ou igual a outro |
    | Maior igual | <=      | Verifica se um valor é menor ou igual a outro |
    +-------------+---------+-----------------------------------------------+")
}
