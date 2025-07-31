use crate::classes::{point, vecteur};

pub struct Operationvect2d{
    vect: vecteur::Vecteur
}

impl Operationvect2d {
    pub fn new(new_vect: vecteur::Vecteur)->Operationvect2d{
        Operationvect2d { vect: new_vect }
    }

    pub fn mult_scalaire(&self,scalaire:f64)->vecteur::Vecteur{
        let currentpoint=self.vect.get_point();
        let point_x=currentpoint.get_x() as f64;
        let point_y=currentpoint.get_y() as f64;
        let new_point=point::Point::new(point_x*scalaire , point_y*scalaire);
        let new_vect=vecteur::Vecteur::new(new_point);
        new_vect
    }    

    pub fn prod_scalaire(&self,vect2:vecteur::Vecteur)->f64{
        let point1=self.vect.get_point();
        let point1_x=point1.get_x();
        let point1_y=point1.get_y();
        let point2=vect2.get_point();
        let point2_x=point2.get_x();
        let point2_y=point2.get_y();
        let new_point_x=point1_x*point2_x;
        let new_point_y=point1_y*point2_y;
        new_point_x+new_point_y
    }

    pub fn normalize(&self)->vecteur::Vecteur{
        let norme_vect=self.vect.get_norme();
        let point=self.vect.get_point();
        let point_x=point.get_x();
        let point_y=point.get_y();
        let f_x=point_x/norme_vect;
        let f_y=point_y/norme_vect;
        let f_point=point::Point::new(f_x, f_y);
        let f_vecteur=vecteur::Vecteur::new(f_point);
        f_vecteur
    }

    pub fn sous_vect(&self,vect1: vecteur::Vecteur)->vecteur::Vecteur{
        let currentpoint=self.vect.get_point();
        let current_x=currentpoint.get_x();
        let current_y=currentpoint.get_y();
        let vect1_pt=vect1.get_point();
        let vect1_pt_x=vect1_pt.get_x();
        let vect1_pt_y=vect1_pt.get_y();
        let new_point=point::Point::new(current_x-vect1_pt_x, current_y-vect1_pt_y);
        let new_vect=vecteur::Vecteur::new(new_point);
        new_vect
    }
}

impl Drop for Operationvect2d{
    fn drop(&mut self) {
    }
}

