use std::env;
mod primeGenerator;
use primeGenerator::PrimeGenerator;

fn main() {
    let args: Vec<String> = env::args().collect();
  
    // Verificar si hay al menos un argumento
    if args.len() < 2 {
        println!("Por favor, proporciona al menos un argumento en la línea de comandos.");
        return;
    }
  
    // Obtener el primer argumento y convertirlo a i64
    let max: i64 = match args[1].parse() {
        Ok(valor) => valor,
        Err(_) => {
            println!("Error: No se pudo convertir el argumento a i64.");
            return;
        }
    };
    let mut prime_generator = PrimeGenerator::new();
    let optimus_prime = prime_generator.get_big_prime(); 
    println!("{}",optimus_prime);
  
  
  }