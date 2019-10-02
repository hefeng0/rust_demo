use std::option::Option;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum IpType {
    V4(u32),
    V6,
}

/*
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
*/

#[derive(Debug)]
enum UsState {
   Alabama,
   Alaska,
}

enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect = Rectangle{width:width1,height:height1};
    println!("the rect is {:?}", rect);
    println!("the rect is {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    let ipType = IpType::V6;
    println!("IpType is {:#?}", ipType);

    let r1 = value_in_cents(Coin::Quarter(UsState::Alaska));
    let r2 = value_in_cents(Coin::Penny);
    println!("r1={} r2={}", r1, r2);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(5) => Some(5+1),
    }
}


