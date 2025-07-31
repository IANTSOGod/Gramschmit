
#[derive(Clone)]
pub struct Point{
    x:f64,
    y:f64,
}

impl Point{
    pub fn new(a:f64,b:f64)->Point{
        Point{x:a,y:b}
    }

    //getters
    pub fn get_x(&self)->f64{
        self.x
    }
    pub fn get_y(&self)->f64{
        self.y
    }

    //setters
    pub fn set_x(&mut self,new_x:f64){
        self.x=new_x;
    }
    pub fn set_y(&mut self,new_y:f64){
        self.y=new_y;
    }

    //methodes
    pub fn show(&self){
        println!("x:{} et y:{}",self.x,self.y);
    }

}

impl Drop for Point{
    fn drop(&mut self) {
    }
}