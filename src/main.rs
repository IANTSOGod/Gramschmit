mod classes;

fn main() {
    //creation d"un point
    let mut point=classes::point::Point::new(2,3);
    point.show();
    point.set_x(1);
    point.set_y(0);
    point.show();
    let x:i8=point.get_x();
    let y:i8=point.get_y();
    println!("{:?} et {:?} depuis getters",x,y);

    //creation d"un vecteur
    let vect=classes::vecteur::Vecteur::new(point);
    println!("La norme de ce vecteur est {:?}",vect.get_norme());
}
