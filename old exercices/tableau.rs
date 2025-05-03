fn main(){

    let tab:[i32;4] = [11,23,19,19];
    //  let _tab:[i32;4] = [11,23,19,19];

    //pour éviter le warning d'une variable non utilisée on rajoute le _ devant la variable

    println!(" le tableau tab {}",tab[0]);
    println!(" le tableau tab {}",tab[1]);
    println!(" le tableau tab {}",tab[2]);
    println!(" le tableau tab {}",tab[3]);


    // si je veux créer une boucle

    for i in 0..tab.len(){
        println!(" le tableau tab {}",tab[i]);

    }
    // Rust permet de boucler directement sur les elements d'un tableau sans avoir besoin des indices
    // &tab => on passe une référence au tableau pour évoter de prendre la possession du tableau entier
    // &elt => itèrer sur des références aux elements du tableau
    for &elt in &tab {
        println!("l element est {}",elt);
    }


    //
    let mut count = 0;
    loop{
        println!("Counter: {}",count);
        count +=1;

        if count == 3{
            break; // on sort de la boucle quand counter atteint 3
        }

    }


}