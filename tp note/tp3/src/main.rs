/*

Générer un nombre aléatoire entre 1 et 100

L’utilisateur doit deviner le nombre

Le programme indique "trop grand", "trop petit", ou "gagné"

Librairie utile : rand

*/

use rand::Rng;
use std::io;

fn generate_random_number() -> u32 {
    let mut rng = rand::rng(); // init le générateur de nombres aléatoires
    rng.random_range(1..=100) // génère un nombre aléatoire entre 1 et 100
}

fn main() {
    let nombre_aleatoire = generate_random_number(); // appel de la fonction

    println!("Indiquer la longueur du mot de passe:"); // demander à l'utilisateur de rentrer un nombre
    let mut nombre = String::new(); // on initialise une variable mutable (modifiable) de type String
    io::stdin().read_line(&mut nombre).unwrap(); // on lit la ligne de l'entrée standard (stdin) et on la stocke dans la variable nombre

    let nombre_parse: u32 = nombre.trim().parse().unwrap_or(0); // on parse le nombre

    // Compare le nombre entré par l'utilisateur avec le nombre aléatoire
    if nombre_parse > nombre_aleatoire{
        println!("Trop grand");
    } else if nombre_parse < nombre_aleatoire{
        println!("Trop petit");
    } else {
        println!("Gagné");
    }

    println!("Le nombre était {}", nombre_aleatoire);
}
