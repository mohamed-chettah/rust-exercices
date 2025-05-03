// enum_forme.rs
use std::f64::consts::PI;
struct Cercle {
    rayon : f64,
}

struct Rectangle{
    largeur: f64,
    hauteur:f64,
}

struct Carre{
    cote: f64,
}


// Définir  enum pour les différentes formes

enum Forme{
    Cercle(Cercle),
    Rectangle(Rectangle),
    Carre(Carre),
}

// implémentations

impl Forme {
    fn surface(&self) -> f64{

        match self{
            Forme::Cercle(c) => PI *c.rayon *c.rayon, // surface du cercle
            Forme::Rectangle(r) =>r.largeur* r.hauteur, // surface du rectangle
            Forme::Carre(c) => c.cote * c.cote, // surface d'un carré
        }
    }
}

fn main (){

    let cercle = Forme::Cercle(Cercle{rayon:6.0});
    let rectangle = Forme::Rectangle(Rectangle{largeur:6.0, hauteur:5.0});
    let carre = Forme::Carre(Carre{cote:4.0});

    println!("surface du cercle est : {}",cercle.surface());
    println!("surface du rectangle est : {}",rectangle.surface());
    println!("surface du carre est : {}",carre.surface());

}



