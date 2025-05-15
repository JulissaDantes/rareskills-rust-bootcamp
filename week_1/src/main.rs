fn function_1(var: u8) {

    println!("In function_1, variable is: {}", var);
}

// Why it doesn't work? function_1 keeps ownership of variable, tf println cannot use it.
// Solution 1: Send a copy of variable to function_1.
// Solution 2: Make variable mutable and return the variable from function_1 into variable.
fn main() {
    //let variable = String::from("Welcome to RustSkills");
    /*Now, replace the String variable with a scalar variable (u32, i32, u64, i64, …) and retest the same code snippet.
    Why does it work? Because scalar values implement the copy trait, tf its sending a copy to function_1 */
    let variable = 1u8;

    function_1(variable);

    println!("In main, variable is: {}", variable);
}