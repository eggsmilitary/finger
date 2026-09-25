// arrays, vectors, tuples, hashmaps

use std::collections::HashMap;  // import

fn main() {
    let arr = ["a", "c", "s", "f"]; // arrays are static, all elements must be of the same type
    println!("{}", arr[2]);

    let mut vect = vec![3, 1, 4];   // vectors are dynamic
    vect.push(1);
    vect.push(5);
    vect.push(9);

    for i in 0..6 {
        print!("{}", vect[i]);
    }
    println!();

    let me = ("eggs", "a programmer", 2009);    // tuples
    println!("{}, im {}, my bd is {}", me.0, me.1, me.2);

    let mut hash = HashMap::new();  // hash maps
    hash.insert("eggsmilitary", "exmilitary");
    hash.insert("eggsellent", "excellent");
    hash.insert("eggsterminator", "exterminator");

    println!("my puns are {}", hash["eggsellent"]);
}
