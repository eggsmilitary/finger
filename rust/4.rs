// loops, whiles, fors, flow control

fn main() {
    let mut count = 0;
    loop {  // it will loop infinitely tho
        println!("{}", count);
        count += 1;
        if count > 10 {break;}
    }   // so its a bit manual

    let mut bool = false;
    while !bool {   // while loops
        println!("a\n");
        bool = true;
    }   // continue exists too

    for i in 1..=100 {  // a..b => a - b - 1, a..=b => a - b
        if i % 15 == 0 {println!("fizzbuzz");}
        else if i % 3 == 0 {println!("fizz");}
        else if i % 5 == 0 {println!("buzz");}
        else {println!("{}", i);}
    }
}
