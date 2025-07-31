mod classes;

//But: trouver <v1,v2> 


fn main() {
    //points initialisants les Vect<u1,u2>
    let pointa0=classes::point::Point::new(3.0, 1.0);
    let pointa1=classes::point::Point::new(5.0, 3.0);

    //Les vecteurs <u1,u2>
    let u1=classes::vecteur::Vecteur::new(pointa0);
    let u2=classes::vecteur::Vecteur::new(pointa1);

    //opérateurs
    let u1_oper=classes::operationvect2d::Operationvect2d::new(u1);    
    let u2_oper=classes::operationvect2d::Operationvect2d::new(u2.clone());
    
    //normalisation u1
    let v1=u1_oper.normalize();
    v1.show();

    //recherche v2
    //recherche projeté orthogonal
    let e1_oper=classes::operationvect2d::Operationvect2d::new(v1);
    let u2_scal_e1=e1_oper.prod_scalaire(u2.clone());
    let p_u2=e1_oper.mult_scalaire(u2_scal_e1 as f64);

    //recherche v2'
    let v2_pr=u2_oper.sous_vect(p_u2);

    //recherche v2
    let v2_pr_op=classes::operationvect2d::Operationvect2d::new(v2_pr);
    let v2=v2_pr_op.normalize();
    v2.show();

    
    

}
