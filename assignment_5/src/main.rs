// Modify the solution to the Section 4 assignment by creating a Trait that has the set_mpg, set_color, and set_top_speed methods. Then create a Motorcycle struct with the same fields as the Car struct: mpg, color, and top_speed. Now implement your Trait on both the Car and Motorcycle struct. Print out the results to confirm a working solution.

// Create a simple print function that uses Generic T. This Generic T will need to implement std::fmt::Debug depending on the values you pass in. Our function takes one parameter of type T. Our function will then print out the value that is passed in.

// fn main() {
//     let mut car = Car{mpg: 10, color: String::from("Yellow"), top_speed: 200};
//     car.set_mpg(20);
//     car.set_color(String::from("Purple"));
//     car.set_top_speed(400);
//     //println!("{:#?}", car);
//     print_value(car);
// }

// #[derive(Debug)]
// struct Car {
//     mpg: u32,
//     color: String,
//     top_speed: u32,
// }

// struct Motorcycle {
//     mpg: u32,
//     color: String,
//     top_speed: u32,
// }

// trait Set_spec {
//     fn set_mpg(&mut self, mpg: u32);
//     fn set_color(&mut self, color: String);
//     fn set_top_speed(&mut self, top_speed: u32);
// }

// impl Set_spec for Car {
//     fn set_mpg(&mut self, mpg: u32){
//         self.mpg = mpg;
//     }
//     fn set_color(&mut self, color: String){
//         self.color = color;
//     }
//     fn set_top_speed(&mut self, top_speed: u32){
//         self.top_speed = top_speed;
//     }
// }

impl Set_spec for Motorcycle {
    fn set_mpg(&mut self, mpg: u32){
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String){
        self.color = color;
    }
    fn set_top_speed(&mut self, top_speed: u32){
        self.top_speed = top_speed;
    }
}

fn main(){

}

// fn print_value<T: std::fmt::Debug> (value:T){
//     println!("{:#?}", value);
// }