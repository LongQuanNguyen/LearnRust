// Create a vector with the values 1, 3, 5, 7, and 9. Then use an iterator and a closure to multiply all of the values by 10 and store the result in another vector. Print out the vector to confirm your results.

fn main() {
    let vec = vec![1,3,5,7,9];
    let result_vec: Vec<i16> = vec.iter().map(|element| element * 10 ).collect();
    println!("{:?}", result_vec);
}
