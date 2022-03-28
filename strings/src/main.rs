fn main() {
    
    // Strings no rust podem ser declaradas de duas formas

    // String
    let name:String = String::from("Ian"); // Variavel do tipo string
    println!("Nome: {name}");


    // &str
    let name2:&str = "Ian Arnold"; // Variavel do tipo &str
    println!("Nome: {name2}");



    // Diferenças   
    // str é o tipo pra guardar textos de uma forma mais bruta.
    // é um array de bytes UTF-8, é imutavel (em partes)

    // String
    //String é uma wrapper para o str e é um tipo por referência 
    // alocado no heap, portanto o tempo de vida tende ser maior e 
    // exige um controle menos sofisticado por parte do programador.
    // É MUTAVEL.
    
}
