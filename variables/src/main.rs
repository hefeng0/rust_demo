fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess value is {}", guess);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The first value of a is {}", a[0]);

    another_function(5, 6);

    let str: String = String::from("hefeng ljh");
    slice_function(&str);
}

fn another_function(x: u32, y: u32) {
    println!("another function x is {}", x);
    println!("another function y is {}", y);
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let s = String::from("name");
    let s2 = s;
    println!("the value of s is {}", s2);
}

fn slice_function(s: & String) {
    let word = first_word(&s[..]);
    println!("word is {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
