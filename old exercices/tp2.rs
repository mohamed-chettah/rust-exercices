// TP2 evaluation
use std::fs::File;
use std::io::{Write, BufReader, Read};

fn main() -> std::io::Result<()> {
    let content = read("filenames.txt")?;
    write("test2.txt", &content)?;

    Ok(())
}

fn read(name_file: &str) -> std::io::Result<String> {
    let file = File::open(name_file)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn write(name_file: &str, content: &str) -> std::io::Result<()> {

    let mut file = File::create(name_file)?; // pour créé
    file.write_all(content.as_bytes())?; // pour écrire
    println!("Le fichier à été créé avec succès !");

    Ok(())
}