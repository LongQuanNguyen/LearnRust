
//Create three variables with the names: val1, val2, and ans. We want to perform a simple operation of generating the modulo of val1 and val2. Set val1 to 5 and val2 to 2. Assign the answer to the ans variable. Before executing your code, what do you think the answer will be?

//Create a vector and put in the values "2, 4, 6, 8, 10". Once you have created the vector perform the following: print out the current values, remove the value 10, add the value 12, and then print the vector back out to confirm your results.

//Create a function called "concat_string". Create a string variable and assign the value "Hello" to it. The function is going to take one argument that is of type string and is going to return a String. Inside this function, concatenate the string " World". Print out the results in main() to confirm your results.

//Create a function called control_flow. This is going to take one argument that is an integer. Based on this integer, print out the following: "The value is one", "The value is greater than 50", "The value is less than 25", or "The value is greater than 25 but less than 50".


// fn main() {
//     let val1 = 5;
//     let val2 = 2;
//     let ans = val1 % val2;
//     println!("{ans}");
// }

// fn main() {
//     let mut vec = vec![2, 4, 6, 8, 10];
//     for value in &vec {
//         println!("{value}");
//     }
//     vec.remove(4);
//     vec.push(12);
//     println!("{:?}", vec);
// }

// fn main() {
//     print!("{}", concat_string("Hello"));
// }

// fn concat_string(str: &str) -> String{
//     let string = str.to_string() + " World";
//     string
// }

// fn main() {
//     control_flow(25);
//     control_flow(50);
//     control_flow(1);
//     control_flow(24);
//     control_flow(26);
//     control_flow(26);
//     control_flow(49);
// }

// fn control_flow(int: i64) {
//     if int == 1 { 
//         println!("The value is one")
//     } else if int > 50 {
//         println!("The value is greater than 50");
//     } else if int < 25 {
//         println!("The value is less than 25")
//     } else if int != 25 && int != 50 {
//         println!("The value is greater than 25 but is less than 50")
//     }
// }

fn main() {}