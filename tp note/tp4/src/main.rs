/*
Envoi d’e-mails depuis le terminal avec Rust
Utiliser une librairie tierce (lettre) pour envoyer des e-mails
Travailler avec les structures, gestion d’erreurs, et I/O terminal
*/

use lettre::{Message, SmtpTransport};
use lettre::Transport;
use std::io;

fn send_email(to: &str, subject: &str, body: &str) {
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body.to_string())
        .unwrap();

    // Open a local connection on port 25 and send the email
    let mailer = SmtpTransport::unencrypted_localhost();
    match mailer.send(&email) {
        Ok(_) => println!("✅ Mail envoyé avec succès !"),
        Err(e) => println!("❌ Erreur lors de l'envoi : {}", e),
    }

}

fn main() {
    println!("Indiquer le destinataire du mail :"); // d
    let mut to = String::new(); // on initialise une variable mutable (modifiable) de type String
    io::stdin().read_line(&mut to).unwrap(); // on lit la ligne de l'entrée standard (stdin) et on la stocke dans la variable nombre

    println!("Indiquer l'objet du mail :"); // d
    let mut object = String::new(); // on initialise une variable mutable (modifiable) de type String
    io::stdin().read_line(&mut object).unwrap(); // on lit la ligne de l'entrée standard (stdin) et on la stocke dans la variable nombre

    println!("Indiquer le message du mail :");
    let mut body = String::new(); // on initialise une variable mutable (modifiable) de type String
    io::stdin().read_line(&mut body).unwrap(); // on lit la ligne de l'entrée standard (stdin) et on la stocke dans la variable nombre

    send_email(&to, &object, &body); // on appelle la fonction send_email avec les paramètres to et object
}
