// functions, return values, scopes

fn main() {
    something();
    println!("{}", ergs("text"));
}   // scopes are the same too, but variables in all blocks only exist in that block. including conditionals

fn something() {
    println!("did something");
}

fn ergs(t: &str) -> &str {  // ret vals have to be specified
    return t;
}
