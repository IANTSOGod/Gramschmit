mod classes;

//But: trouver <v1,v2> 


fn main() {
    //points initialisants les Vect<u1,u2>
    let pointa0=classes::point::Point::new(2, 0);
    let x0=pointa0.get_x();
    let x1=pointa0.get_y();
    let pointa1=classes::point::Point::new(3, 1);

    //Les vecteurs <u1,u2>
    let u1=classes::vecteur::Vecteur::new(pointa0);
    let u2=classes::vecteur::Vecteur::new(pointa1);

    //on a v1=u1

    //recherche e1
    let norme_u1=u1.get_norme();
    let v1_x=x0 as f64;
    let v1_y=x1 as f64;
    println!("V1 a x:{} et y:{}",v1_x/norme_u1,v1_y/norme_u1);
}
