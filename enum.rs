

// enum.rs

enum Direction {
    Haut,
    Bas,
    Gauche,
    Droite,
}

fn main(){

    let direction = Direction::Haut;

    match  direction {
        Direction::Haut => println!(" vers le haut "),
        Direction::Bas => println!(" vers le bas  "),
        Direction::Gauche => println!(" vers la gauche "),
        Direction::Droite => println!(" vers la droite "),
        _=> println!("Aucune direction ! failed !!")

    }
}
