// struct Shape{
//     width : i32,
//     height : i32,
// }

// trait calculate{
//     fn is_square(&self)->bool;
// }

// impl calculate for Shape{
//     fn is_square(&self) -> bool{
//         return self.width == self.height
//     }   
// }

// fn main() {
//     let shapeA = Shape{width : 10, height : 10};
//     println!("Shape is square ? {}", shapeA.is_square());
//     // Shadowing
// }


extern crate chrono;
use chrono::prelude::*;

#[derive(Debug)]

struct ToDo{
    name : String,
    is_done : bool,
    date_created : String
}

impl ToDo{
    fn new(name : String) -> ToDo{
        ToDo {
            name : name,
            is_done : false,
            date_created : Utc::now().to_string()
        }
    }
    fn print(self){
        println!("ToDo : {}, Date Created : {}", self.name, self.date_created)
    }
}

struct List{
    list : [ToDo]
}

fn main(){
    let first_todo = ToDo::new("Washing dishes".into());
    first_todo.print();
}
