// Les input et output

use std::io;
use std::io::Write;

// fn main() {
//     print!("Veuillez saisir une valeur : ");
//     io::stdout().flush().unwrap();
//
//     let mut valeur = String::new();
//     io::stdin().read_line(&mut valeur).unwrap();
//
//     println!("Vous avez saisi : {}", valeur);
// }


// fn main() {
//
//     print!("Veuillez saisir une valeur : ");
//     io::stdout().flush().unwrap();
//
//     let mut valeur = String::new();
//     io::stdin().read_line(&mut valeur).unwrap();
//
//     println!("Vous avez saisi : {}", valeur);
//
//     let x = 5;
//     let y = 10;
//     let z = x + y;
//
//     println!("La somme de {} et {} est : {}", x, y, z);
// }

// use std::io::{self, Write};
fn main() {
    let msg = "Bonjour désolé pour le bug !";

    print!("{}",msg);
    // s'assurer de l'écriture immédiate du message
    io::stdout().flush().unwrap();
    println!("{}", msg);
}