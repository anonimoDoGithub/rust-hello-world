fn main() {
    let velocidade1: u16 = 81;

    if velocidade1 < 100 && velocidade1 >= 80 {
        println!("Diminua a velocidade!")
    }else if velocidade1 >= 100 {
        println!("Multa por excesso de velocidade!")
    }else {
        println!("Sem excesso de velocidade!")
    }
    
}