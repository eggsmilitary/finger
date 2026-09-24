// variables and datatypes

fn main() {
    let me = "eggs";
    let eggs = "yum";
    println!("{} are {}my", me, eggs);

    // variables cant be changed after definition
    // but there is mut

    let mut i = 100;
    i += i;

    println!("{}", i);  // w

    // rust will automatically determine data types, but they can be explicit
    let num: i32 = 2009;    // signed & unsigned, not sure about the size
    let float: f64 = 3.14159;   // float
    let char: char = 'A';   // single quotes only
    let str: &str = "eggs"; // double quotes only
    let bool: bool = true;  // bool
}
