// ownership and borrowing

fn main() {
    let a = 1;
    let b = a;
    // simple types are alright, but strings will cause problems

    let c = String::from("hoi");
    let d = c;
    // a no longer owns the value
    // move on or just clone it
    let e = String::from("im temmie");
    let f = e.clone();

    // ownership and stuff, and borrowing...
    let mut g = String::from("hi, im bob");
    let h = &g;     // to access a value without taking ownership (reference)

    let i = &mut g  // to change a value through a reference
}
