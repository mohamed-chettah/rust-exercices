fn calcul_aire(largeur:f64, hauteur:f64) ->f64{
    largeur * hauteur
}

fn main() {
    let largeur = 30.0;
    let hauteur = 50.0;

    println!("L'aire du rectangle est de : {}", calcul_aire(largeur, hauteur));
}