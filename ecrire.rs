use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("test.txt")?; // pour créé
    file.write_all(b"Bonjour, comment ca va ?")?; // pour écrire
    println!("Le fichier à été créé avec succès !");

    // b est byte string on utilise lorsque on travaille avec des données binaires
    Ok(())
}