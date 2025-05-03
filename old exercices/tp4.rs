// Gestion d'un compte bancaire
// systeme de gestion de compte bancaire
// avec un menu de 4 options
// 1.Consulter le solde
// 2.Effectuer le dépot
// 3.Effectuer le retrait
// 4.Quitter
// Le solde du compte est initialisé à 0
use std::fs::File;
use std::io::{self, Write, BufReader, Read};

struct CompteBancaire {
    solde: f64,
}

impl CompteBancaire {
    fn consulter_solde(&mut self) {
        // lecture du fichier solde.txt
        let file = File::open("solde.txt").unwrap();
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content);
        self.solde = content.trim().parse().unwrap();
        println!("Votre solde est de : {}", self.solde);
    }
    fn effectuer_depot(&mut self, montant: f64) {
        self.solde += montant;
        println!("Dépot effectué avec succès !");
        println!("Votre solde actuel est de : {}", self.solde);
        create_file_solde(self.solde);
    }
    fn effectuer_retrait(&mut self, montant: f64) -> Result<(), io::Error> {
        if self.solde >= montant {
            self.solde -= montant;
            println!("Retrait effectué avec succès !");
            create_file_solde(self.solde)
        }
        else{
            println!("Solde insuffisant !");
            Ok(())
        }
    }
}

fn create_file_solde(solde: f64) -> io::Result<()> {
    let mut file = File::create("solde.txt")?; // pour créé
    file.write_all(format!("{}", solde).as_bytes())?; // pour écrire
    Ok(())
}

fn main(){
    let mut compte = CompteBancaire{solde: 0.0};
    let mut choix = 0;
    let mut montant = 0.0;
    loop {
        println!("Menu : ");
        println!("1. Consulter le solde");
        println!("2. Effectuer le dépot");
        println!("3. Effectuer lxe retrait");
        println!("4. Quitter");
        println!("Entrez votre choix : ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        choix = input.trim().parse().unwrap();
        match choix {
            1 => compte.consulter_solde(),
            2 => {
                println!("Entrez le montant à déposer : ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                montant = input.trim().parse().unwrap();
                compte.effectuer_depot(montant);
            },
            3 => {
                println!("Entrez le montant à retirer : ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                montant = input.trim().parse().unwrap();
                compte.effectuer_retrait(montant);
            },
            4 => break,
            _ => println!("Choix invalide !"),
        }
    }
}


