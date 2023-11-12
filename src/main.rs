fn main() {
    let nome = "Sergio";
    let sobrenome: &str = "Felipe";
    let idade: i8 = 37;
    let altura: f32 = 1.75;
    let estudando: bool = true;
    let mut pais = "Brazil"; 
    //pais = "Canadá";

    println!("Seja bem vindo, {} {}!", nome, sobrenome);
    println!("Sua idada atual é: {}.", idade);
    println!("Sua altura atual é: {}.", altura);
    println!("No momento você está aprendendo sobre: [{}] RUST.", estudando);
    println!("Seu país de atual é: {}.", pais);
}

/* Variáveis
    Em RUST todas as variáveis são por padrão IMUTAVEIS ou seja não podem ter seus valores REATRIBUIDOS.
    *Porém, caso seja necessário podemos declarar a variável como MUTAVEL usando  "mut" antes do nome da variável.
    Exemplo: let mut pais = "Brazil";
    ----
    Não precisamos explicitar qual o tipo de dado, em nossas variáveis.
    O Rust é uma linguagem de programação que suporta inferência de tipo.
    *Porém a inferência de tipo no Rust ajuda a tornar o código mais conciso e legível,
    enquanto ainda mantém a segurança de tipo que é uma característica fundamental da linguagem.
*/

/* 

*/