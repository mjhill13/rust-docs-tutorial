const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    mutability;
    shadowing;
    shadowing_types;
    shadowing_types_mut_error
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");        
}

fn shadowing_types() {
    let spaces = "   ";
    let spaces = spaces.len();
}

fn shadowing_types_mut_error() {
/*  This will cause a type error because the variable is declared mutable, but not a shadow variable.
    We cannot change the type of mutable variables
    
    let mut spaces = "   ";
    spaces = spaces.len();
*/
}