// Create a struct called Car with the fields: mpg, color, and top_speed. Once the struct is created, implement the following methods: set_mpg, set_color, and set_top_speed. Once you have created these methods, create a car, provide it default values, and then set the mpg, color, and top speed and then print them out.

#[derive(Debug)]
struct Car {
    mpg: u32,
    color: String,
    top_speed: u32,
}

impl Car {
    fn set_mpg(&mut self, mpg: u32) {
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: String) {
        self.color = color;
    }

    fn set_top_speed(&mut self, top_speed: u32) {
        self.top_speed = top_speed;
    }
}
fn main() {
    let mut car1 = Car{mpg: 10, color: String::from("Purple"), top_speed: 200};
    println!("{} {} {}", car1.mpg, car1.color, car1.top_speed);
    car1.set_mpg(11);
    car1.set_color(String::from("Yellow"));
    car1.set_top_speed(300);
    println!("{} {} {}", car1.mpg, car1.color, car1.top_speed);
    println!("{:#?}", car1);

}
