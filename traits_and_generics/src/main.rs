// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// Trait is similiar to interface
trait Overview {
    fn over_view(&self) -> String{
        String::from("Rust Course") // Default method
    }
}

struct Course {
    headline: String,
    instructor: String,
}

struct AnotherCourse {
    headline: String,
    instructor: String,
}

impl Overview for Course{
    // fn over_view(&self) -> String{
    //     format!("{} , {}", self.instructor, self.headline)
    // }
}

impl Overview for AnotherCourse{
    fn over_view(&self) -> String{
        format!("{}, {}", self.instructor, self.headline)
    }
}

fn main() {
    // let coord = Point {x: 5.0, y: 5.0};
    // let coord1 = Point { x: 'x', y: 5};

    let course1 = Course{headline: String::from("Headline!"), instructor: String::from("Tony")};
    let course2 = AnotherCourse{headline: String::from("Another Headline!"), instructor: String::from("Another Tony")};

    // println!("{}", course1.over_view());
    // println!("{}", course2.over_view());

    call_overview(&course1);
    call_overview(&course2);
}

// fn call_overview(item: &impl Overview){
//     println!("Overview: {}", item.over_view());
// }

// More concise implement than above
fn call_overview<T: Overview> (item: &T){
    println!("Overview: {}", item.over_view());
}

// fn overview(item1: &impl Overview, item2: &impl Overview)
// fn overview<T: Overview>(item1: &T, item2: &T)
// fn overview(item1: &impl Overview + AnotherTrait)
// fn overview<T: Overview + AnotherTrait>( item1: &T,item2: &T)

