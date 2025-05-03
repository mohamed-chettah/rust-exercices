/*

Demander à l’utilisateur la longueur du mot de passe
Générer un mot de passe avec lettres, chiffres, caractères spéciaux
Afficher le mot de passe à l’écran
Librairie utile : rand
Bonus : Option pour exclure certains caractères (l, 1, 0, etc.)

*/

use rand::Rng;
use std::io;

fn generer_mot_de_passe(taille: usize, exclure: &[char]) -> String {
    let mut rng = rand::rng(); // initialiser le générateur de nombres aléatoires
    let caracteres = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+"; // liste des caracteres qu'on peut utiliser

    // on transforme la chaîne de caractères en un vecteur de caractères (pour accéder à chaque caractère)
    let caracteres: Vec<char> = caracteres
        .chars() // itérer sur chaque caractère
        .filter(|c| !exclure.contains(c)) // c = le caractère courant pendant le parcours avec .chars() : filtrer les caractères à exclure
        .collect(); // On récupère les caractères restants dans un vecteur

    (0..taille)
        .map(|_| caracteres[rng.random_range(0..caracteres.len())]) // la on rng génère un nombre aléatoire entre 0 et la taille du vecteur de caractères et est utilisé comme index pour accéder à un caractère aléatoire
        .collect()
}

fn main() {
    println!("Indiquer la longueur du mot de passe:");
    let mut longueur = String::new();
    io::stdin().read_line(&mut longueur).unwrap();

    let longueur: usize = longueur.trim().parse().unwrap_or(8); // par défaut 8 si erreur

    let exclure = vec!['l', '1', '0'];

    let mot_de_passe = generer_mot_de_passe(longueur, &exclure);
    println!("Mot de passe généré : {}", mot_de_passe);
}

