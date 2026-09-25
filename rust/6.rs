// strings and methods

fn main() {
    // dynamic strings are different
    let a: &str = "this string cannot be changed after declaration";

    let b = "this string can be changed".to_string();
    let c = String::from("this is another way");    // very cpp-esque

    let mut d = "half of".to_string();
    d.push_str(" the string");  // append string
    d.push('.');    // append char
    println!("{}\n{}\n{}\n{}", a, b, c, d);

    let all = format!("{} {} {} {}", a, b, c, d);   // string concatenation
    // or use the + operator
    println!("{}", all.len());  // str len
}
