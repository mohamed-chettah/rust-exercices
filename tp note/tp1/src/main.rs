/*
Créer une struct Produit { nom: String, quantite: u32 }
Permettre d’ajouter, lister et modifier un produit dans un inventaire (stock en mémoire)
Menu en boucle avec des choix simples (ajouter, supprimer, afficher)
Bonus : Sauvegarder l'inventaire dans un fichier texte (I/O)
*/
use std::io;
use std::io::Write;
struct Produit {
    nom: String,
    quantite: u32,
}

impl Produit {
    fn ajouter(nom: String, quantite: u32) -> Self {
        Self { nom, quantite }
    }

    fn afficher(&self) {
        println!("Produit: {}, Quantité: {}", self.nom, self.quantite);
    }

    fn modifier(&mut self, quantite: u32) {
        self.quantite = quantite;
    }

}
fn main() {
    let mut inventaire: Vec<Produit> = Vec::new();

    loop {
        println!("\nMenu:");
        println!("1. Ajouter un produit");
        println!("2. Supprimer un produit");
        println!("3. Modifier un produit");
        println!("4. Afficher les produits");
        println!("5. Quitter");
        print!("Choix: ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        let choix = choix.trim();

        match choix {
            "1" => {
                println!("Nom du produit:");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).unwrap();
                let nom = nom.trim().to_string();

                println!("Quantité:");
                let mut quantite_str = String::new();
                io::stdin().read_line(&mut quantite_str).unwrap();
                let quantite: u32 = quantite_str.trim().parse().unwrap_or(0);

                inventaire.push(Produit::ajouter(nom, quantite));
                println!("Produit ajouté.");
            }

            "2" => {
                println!("Nom du produit à supprimer:");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).unwrap();
                let nom = nom.trim();

                // Supprimer si le nom correspond
                inventaire.retain(|p| p.nom != nom);
                println!("Produit supprimé si trouvé.");
            }

            "3" => {
                println!("Nom du produit à modifier:");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).unwrap();
                let nom = nom.trim();

                // Trouver le produit et modifier sa quantité
                for p in &mut inventaire {
                    if p.nom == nom {
                        println!("Nouvelle quantité:");
                        let mut quantite_str = String::new();
                        io::stdin().read_line(&mut quantite_str).unwrap();
                        let quantite: u32 = quantite_str.trim().parse().unwrap_or(0);
                        p.modifier(quantite);
                        println!("Produit modifié.");
                    }
                }
            }

            "4" => {
                println!("Inventaire:");
                for p in &inventaire {
                    p.afficher();
                }
            }

            "5" => {
                println!("Au revoir !");
                break;
            }

            _ => println!("Choix invalide."),
        }
    }
}