fn main() {
    println!("Hello, world!");
    //Statements and expressions
    //Instructions that perform some action and do not return a value
    //Expressions evaluate to a resultant value
    let y = 6; // A statement that creates a value
    // Functions definitions are also statements
    // Statements do not return values => one cant assign a let
    // statement to another variable
    //
    // Expressions evaluate to a values, and they can be part of 
    // statements
    let x = {
        let z = 3;
        z + 1
    };

    let five = five();
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("Value: {value} {unit_label}");
}

//Functions with return values
fn five() -> i32 {
    5
}


