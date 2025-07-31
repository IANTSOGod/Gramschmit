use crate::classes::point;

#[derive(Clone)]
pub struct Vecteur{
    point: point::Point
}

impl Vecteur{
    pub fn new(new_point: point::Point)->Vecteur{
        Vecteur { point: new_point }
    }

    //getters
    pub fn get_point(&self)->point::Point{
        let newpoint=point::Point::new(self.point.get_x(), self.point.get_y());
        newpoint
    }
    //setters
    pub fn set_point(&mut self,new_point: point::Point){
        self.point=new_point
    }

    pub fn get_norme(&self)->f64{
        let x=self.point.get_x()*self.point.get_x();
        let y=self.point.get_y()*self.point.get_y();
        let sum=x+y; 
        sum.sqrt()
    }

    pub fn show(&self){
        let point=self.get_point();
        let point_x=point.get_x();
        let point_y=point.get_y();
        println!("On a le vecteur ({},{})",point_x,point_y)
    }
}

impl Drop for Vecteur{
    fn drop(&mut self) {
    }
}