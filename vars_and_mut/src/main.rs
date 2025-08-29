fn main() {
    //Cannot assign twice to immutable variables
    //Rust compiler guarantees that when a variables is
    //stated to be immutable, it IS immutable
    let x = 5;
    println!("The value of x is: {x}");
    //x = 6; cannot assign to const valuable
    println!("The value of x is: {x}");

    //Mutability
    let mut y = 1;
    println!("The value of x is: {y}");
    y = 9;
    println!("The value of x is: {y}");

    //Constants
    //Constants are values that are bound to a name and are not allowed to change
    //There are a few differences between constants and variables
    //The keyword mut is not allowed

    //they are valid for the entire time a program runs, within the scope in which
    //they are declared
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //Rust's naming convention

    //Shadowing
    //A variable that overshadows another, taking any uses of the variable name
    //to itself until either it itself is shadowed or the scope ends

    let z = 5;
    let z = y + 2;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}")
        //Shadowing is different from marking a variable as mut: we'll get a compile
        //time error if we try to reassign to this variable without using the let keyword:
        //by using let, we can performa a few transformations on a value but have the variable
        //be immutable after those transformations have been completed
        //
        //We are effectively creating a new variable when we use the let keyword
    }
    println!("The value of z: {z}")

}
