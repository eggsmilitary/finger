// consts, operators, conditionals

fn main() {
    const BD: u32 = 2009;   // consts require a type
    println!("i am {} years old in {}!", 2026 - BD, 2026);
    // operators are the same as any other lang
    // booleans too blah blah

    if 2026 - BD > 18 {
        println!("i am now unc :_(");
    }   else {
        println!("infant");
    }   // if, else, elif, match, seen it a thousand times
    // also for some reason if you use conditionals to define a variable,
    // all results must be the same data type

    // theres no conditional shorthand tho...
    let day = 1;
    match day {
        1 | 2 | 3 | 4 => println!("i sleep"),
        5 => println!("i sleep"),
        6 => println!("i sleep"),
        7 => println!("i sleep"),
        _ => println!("i still sleep"),
    }   // can return the result too ig
}
