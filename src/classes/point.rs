
#[derive(Clone)]
pub struct Point{
    x:i8,
    y:i8,
}

impl Point{
    pub fn new(a:i8,b:i8)->Point{
        Point{x:a,y:b}
    }

    //getters
    pub fn get_x(&self)->i8{
        self.x
    }
    pub fn get_y(&self)->i8{
        self.y
    }

    //setters
    pub fn set_x(&mut self,new_x:i8){
        self.x=new_x;
    }
    pub fn set_y(&mut self,new_y:i8){
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