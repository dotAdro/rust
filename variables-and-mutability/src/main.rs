#![allow(unused_variables)] // instead of method. applies to all in the file

const TAX_RATE: &str = "always bad";

fn main() {
    println!("Tax rate is: {TAX_RATE}");

    let fruits = fruits();
    println!("fruits: {fruits}");

    immutable();
    variable_shadowing();
    scopes();
    meters();
    compiler_directives();
}

fn fruits() -> i32 {
    let apples_in_garden = 50;
    println!("apples: {}", apples_in_garden);

    let oranges = 14 + 6;
    println!("oranges: {}", oranges);

    let bananas = 14 + 6;
    println!("bananas: {0} is {0}", bananas);

    let _coconuts = 50; // '_' allows for unused

    apples_in_garden + oranges
}

fn immutable() {
    let _gym_reps = 10;

    // gym_reps = 15; // immutability enforced

    let mut _mutable_gym_reps = 5; // needs the mut. never f use it. bad design
    _mutable_gym_reps = 50;
}

fn variable_shadowing() {
    let _grams_of_protein = "15.00";
    let _grams_of_protein = 15.73;

    let _compounded_grams = _grams_of_protein + 15.00;
}

fn scopes() {
    let _global = 10;

    {
        let _nested = _global + 5;
        println!("nested is accessible: {_nested}");
    }

    // println!("nested not accessible {_nested}"); // will not compile
}

type Meters = i32; // alias

fn meters() {
    let _meters: Meters = 30;
    println!("meters: {}", _meters);
}


// #[allow(unused_variables)] // instead of 'local' allowing
fn compiler_directives() {
    // #[allow(unused_variables)] // instead of '_' -> similar to 'suppress_warnings'
    let meters: Meters = 30;

    let more_meters: Meters = 30;
}
