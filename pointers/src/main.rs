fn main() {
    let t = (12, "eggs"); // created on the stack
    let b = Box::new(t); // box created on the heap, but b was stored on the stack
    println!("{:?}", b);
}
