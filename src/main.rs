fn main() {
    let pais: &str = "Brazil";
    let sigla: char = 'S';
    let ano: i16 = 2023;
    let populacao: f32 = 219.4;
    let povo: bool = true;
    
    println!("Seja bem vindo, ao {}!", pais);
    println!("Ano atual, {}.", ano);
    println!("População atual, {} milhões de habitantes.", populacao);
    println!("Um povo trabalhador, 100% {} {}.", povo, sigla);
    
}

/* Tipos de Dados
    - INTEIROS - i32(32bits)Default
    signed   - i8(8bits)|i16(16bits)|i32(32bits)|i64(64bits)|i128(128bits) *Permitem números positivos e negativos!
    unsigned - u8(8bits)|u16(16bits)|u32(32bits)|u64(64bits)|u128(128bits) *Não permite números negativos!

    - FLUTUANTES - f32(32bits)
    Existem dois tipos de pontos flutuantes: f32(32bits) e f64(64bits)
    f64 - utilizado para DUPLA PRECISÃO;
    f32 - utilizado para SINGLE PRECISÃO;

    - BOOLEANS - true ou false
*/