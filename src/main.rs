mod classes;
use std::process::{Command, Stdio};
use std::io::Write;

//But: trouver <v1,v2> 

fn plot_vectors_from_points(
    u1_point: &classes::point::Point,
    u2_point: &classes::point::Point,
    v1_point: &classes::point::Point,
    v2_point: &classes::point::Point,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut gnuplot = Command::new("gnuplot")
        .stdin(Stdio::piped())
        .spawn()?;

    let stdin = gnuplot
    .stdin
    .as_mut()
    .unwrap();

    writeln!(stdin, "set terminal qt persist")?;
    writeln!(stdin, "set grid")?;
    writeln!(stdin, "set xlabel 'X'")?;
    writeln!(stdin, "set ylabel 'Y'")?;
    writeln!(stdin, "set xrange [-5:5]")?;
    writeln!(stdin, "set yrange [-5:5]")?;
    writeln!(stdin,"set size ratio -1")?;
    writeln!(stdin, "set title 'Orthogonalisation de Gram-Schmidt'")?;
    writeln!(stdin, "set xzeroaxis")?;
    writeln!(stdin, "set yzeroaxis")?;


    writeln!(stdin, "plot '-' with vectors linecolor rgb 'blue' linewidth 3 title 'u1', \\")?;
    writeln!(stdin, "     '-' with vectors linecolor rgb 'blue' linewidth 3 title 'u2', \\")?;
    writeln!(stdin, "     '-' with vectors linecolor rgb 'red' linewidth 3 title 'v1', \\")?;
    writeln!(stdin, "     '-' with vectors linecolor rgb 'red' linewidth 3 title 'v2'")?;

    // Tracage des points
    writeln!(stdin, "0 0 {} {}", u1_point.get_x(), u1_point.get_y())?;
    writeln!(stdin, "e")?;

    writeln!(stdin, "0 0 {} {}", u2_point.get_x(), u2_point.get_y())?;
    writeln!(stdin, "e")?;

    writeln!(stdin, "0 0 {} {}", v1_point.get_x(), v1_point.get_y())?;
    writeln!(stdin, "e")?;

    writeln!(stdin, "0 0 {} {}", v2_point.get_x(), v2_point.get_y())?;
    writeln!(stdin, "e")?;

    writeln!(stdin, "pause -1")?;
    gnuplot.wait()?;
    Ok(())
}

fn main() {
    //points initialisants les Vect<u1,u2>
    let pointa0=classes::point::Point::new(3.0, 3.0);
    let pointa1=classes::point::Point::new(0.0, 5.0);

    //Les vecteurs <u1,u2>
    let u1=classes::vecteur::Vecteur::new(pointa0);
    let u2=classes::vecteur::Vecteur::new(pointa1);

    //opérateurs
    let u1_oper=classes::operationvect2d::Operationvect2d::new(u1.clone());    
    let u2_oper=classes::operationvect2d::Operationvect2d::new(u2.clone());
    
    //normalisation u1
    let v1=u1_oper.normalize();
    v1.show();
    let nv1=v1.get_norme();
    println!("La norme de v1 est {}",nv1);

    //recherche v2
    //recherche projeté orthogonal
    let e1_oper=classes::operationvect2d::Operationvect2d::new(v1.clone());
    let u2_scal_e1=e1_oper.prod_scalaire(u2.clone());
    let p_u2=e1_oper.mult_scalaire(u2_scal_e1 as f64);

    //recherche v2'
    let v2_pr=u2_oper.sous_vect(p_u2);

    //recherche v2
    let v2_pr_op=classes::operationvect2d::Operationvect2d::new(v2_pr);
    let v2=v2_pr_op.normalize();
    v2.show();
    let nv2: f64=v2.get_norme();
    println!("La norme de v2 est {}",nv2);
    
    //traçage plot
    match plot_vectors_from_points(&u1.get_point(), &u2.get_point(), &v1.get_point(), &v2.get_point()) {
        Ok(_)=>println!("Tracé"),
        Err(_)=>{
            println!("Erreur de Tracé")
        }
    }    

}
