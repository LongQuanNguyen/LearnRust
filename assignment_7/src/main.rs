fn main() {
    //Question 1: Create a variable on the stack and a variable on the heap. Multiply their values and print out the results.


    //Question 2: Create a variable that holds a String

    {
        //Create a reference counting smart pointer that points to the above String.

        
        //Print out how many references the smart pointer has.

        //Code block
        {
            //Create another reference counting smart pointer that points to our first smart pointer

            //Print out how many references each smart pointer has
        }
        //What value is dropped here?
        //Print out how many references out first smart pointer has


    } //What value is dropped here?
    //Comment out the line below. What do you think will happen when you try to run the program now?
    //println!("rc_value: {}", rc_value);
}