use crate::classes::point;

pub struct Vecteur{
    point: point::Point
}

impl Vecteur{
    pub fn new(new_point: point::Point)->Vecteur{
        Vecteur { point: new_point }
    }

    pub fn get_norme(&self)->f64{
        let x=self.point.get_x().pow(2);
        let y=self.point.get_y().pow(2);
        let sum=x+y; 
        let norme=sum as f64;
        norme.sqrt()
    }
}

impl Drop for Vecteur{
    fn drop(&mut self) {
        println!("Instance de Vecteur détruite")
    }
}