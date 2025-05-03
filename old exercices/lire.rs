// Lecture à partir d'un fichier
// Pour lire à partir d'un fichier, il faut ouvrir le fichier et lire son contenu. Voici un exemple de code qui lit le contenu d'un fichier texte :
// dans ce cas on utilise Read et BufReader
// BuffRead crée un lecteur tamponné pour l'amélioration des performances

use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let file = File::open("filenames.txt")?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    println!("{}", content);

    Ok(())
}