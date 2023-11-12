fn main() {
    let nome = "Sergio";
    let sobrenome: &str = "Felipe";
    let idade: i8 = 37;
    let altura: f32 = 1.75;
    let estudando: bool = true;

    println!("Seja bem vindo, {} {}!", nome, sobrenome);
    println!("Sua idada atual é: {}.", idade);
    println!("Sua altura atual é: {}.", altura);
    println!("No momento você está aprendendo sobre: [{}] RUST.", estudando);
}
