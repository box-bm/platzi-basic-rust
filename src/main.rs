fn main() {
    let mut nombres: Vec<String> = Vec::new();

    for _i in 0..3 {
        println!("Por favor introduce un nombre: ");
        let mut nombre = String::new();
        std::io::stdin().read_line(&mut nombre).unwrap();

        nombres.push(nombre);
    }

    for nombre in nombres {
        println!("El nombred es: {}", nombre);
    }
    
    let hola = ["H", "O", "L", "A"];
    for letter in hola {
        println!("Letra: {}", letter);
    }
}
