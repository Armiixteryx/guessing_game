//Programación en Rust.
//Esta es una pequeña descripción del ejercicio del cap. 2 de la segunda edición de "El Libro".
//Link: https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html

//Para poder utilizar números aleatorios.
//Previamente hay que añadir la dependencia en el archivo Cargo.toml
extern crate rand;

use std::io; //Entrada y salida.
use std::cmp::Ordering; //Para usar con match. Es de tipo enum.
use rand::Rng; //Generador de números aleatorios.

fn main() {
    println!("¡Adivina el número!");

    //thread_rng() inicializa el generador de números aleatorios.
    //gen_range() genera un número aleatorio entre dos valores.
    //gen_range() no es inclusivo en el extremo derecho.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        
        println!("Por favor, introduce tu número.");

        let mut guess = String::new();

        //read_line() permite ingresar datos por la entrada estándar.
        //read_line() necesita una referencia a una variable mutable, donde se guardará la entrada.
        //expect() permite mostrar un mensaje en caso de error súbito.
        //expect() trabaja con el tipo de dato Result.
        io::stdin().read_line(&mut guess)
            .expect("Fallo al leer la entrada.");

        //trim() elimina espacios al inicio y al final en una variable string.
        //parse() en strings convierte un string a un número.
        //parse() trabaja con varios tipos de datos numéricos.
        //Necesito indicar al compilador qué tipo de dato espero.
        //Con u32 el compilador sabrá cuál es el tipo de dato de la variable.
        //match, Ok(num) y Err(_) son una forma de manejo de errores.
        //Este manejo funciona con el Result devuelto por parse().
        let guess: u32 = match guess.trim().parse() {
        	Ok(num) => num, //Se retorna el dato original de parse() si la conversión funcionó.
        	Err(_) => {
        		//Err(_) trata indiscriminadamente cualquier tipo de error devuelto.
        		println!("Has ingresado algo no válido. Intenta otra vez.");
        		continue;
        	}
        };
        //NOTA: La nueva declaración de guess "enmascara" la anterior. Esto es código válido.

        println!("Has adivinado: {}", guess);

        //cmp() compara guess con secret_number.
        //cmp() toma una referencia con la variable a comparar.
        //cmp() devuelve una variante de Ordering:
        //Con match se decide qué hacer con ese retorno.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("¡Muy pequeño!"),
            Ordering::Greater => println!("¡Muy grande!"),
            Ordering::Equal => {
            	println!("¡Has ganado!");
            	break;
            }
        }
    }
}