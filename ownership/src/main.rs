fn main() {
    // // MOVE
    // let x = vec!["tony".to_string()];
    // let y = x;
    // let z = y;
    // // println!("{:?}",x); // wont work since value got move on line 3
    // // println!("{:?}",y); // wont work since value got move on line 4
    // println!("{:?}", z);

    // // CLONE (deep copy)
    // let x = vec!["tony".to_string()];
    // let y = x.clone();
    // let z = y.clone();
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    // // COPY
    // // support: all int, all float, boolean, char and tuple if all component is supported copy
    // let x = 1;
    // let y = x;
    // println!("{} {}", x, y);

    // let s = String::from("takes");
    // take_onwership(s); // give string "takes" owneship to function
    // // println!("{}", s); // wont work since ownership move to fn take_ownership

    // let val = 1;
    // make_copy(val);
    // println!("{}", val); // works here since int supported copy trait

    // let str1: String = give_ownereship();
    // // println!("{}", str1);

    // let str3: String = take_and_give(str1);
    // // println!("{}", str1); // wont work since take_and_give takes the ownership of given belongs str1 and gives it to str3
    // println!("{}", str3); 

    // if true {
    //     let str4 = str3; // ownership move inside flow control as well
        
    // } else {
    //     let str5 = str3;
    // }

    // let mut str11 = String::from("Tyler");
    // let mut str22: String;

    // loop {
    //     str22 = str11; // wont work on the secon iteration since the ownership is in str11 in the 1st
    // }

    // Mutable reference/ borrowing
    let mut s = String::from("hello");
    change_string(&mut s);
    println!("{}", s);
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn take_onwership(s: String) {
//     let strin = s;
//     println!("{}", strin);
// }

// fn make_copy(one: i32) {
//     let val1 = one;
//     println!("{}", val1);
// }

// fn give_ownereship() -> String {
//     "given".to_string() // this function owns "given"
// }

// fn take_and_give(str2: String) -> String {
//     str2
// }