struct MessageTexte{
    content: String,
}

struct MessageFichier{
    nom_fichier: String,
    taille: u64,
}

enum Message {
    Texte(MessageTexte),
    Fichier(MessageFichier)
}

impl Message {
    fn afficher(&self){
        match self{
            Message::Texte(t) => println!("Message texte : {}",t.content),
            Message::Fichier(f) => println!("Message fichier : {} de taille : {}",f.nom_fichier,f.taille),
        }
    }
}

fn main(){
    let msg1 = Message::Texte(MessageTexte{content: String::from("Bonjour tout le monde")});
    let msg2 = Message::Fichier(MessageFichier{nom_fichier: String::from("fichier.txt"), taille: 1024});

    msg1.afficher();
    msg2.afficher();
}