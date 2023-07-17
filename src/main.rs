// use std::io::{self, Write};
// use std::collections::*;

mod vec;
mod string;
mod hashmap;
mod traits;
mod lifetime;
mod iterator;

fn main() {
    string::string();
    hashmap::hashmap();
    lifetime::get_longest();

    iterator::iterator_sum();

    // vec::vec();

    // calculate_area();
    // let home = IpAddrKind::V4(127, 0, 0, 1);
    // let loopback = IpAddrKind::V6(String::from("::1"));

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // let q = IpAddrKind::V4(111, 22, 11, 11);
    // println!("{:?}", q);
}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//         None => None,
//     }
// }

// #[derive(Debug)]
// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
//     OTHER {x: u32, y: u32},
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     length: u32,
// }

// impl Rectangle {
//     // 可以获得所有权, 借用, 或者可变借用
//     fn area(&self) -> u32 {
//         self.width * self.length
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.length > other.length
//     }

//     fn square(size: u32) -> Rectangle {
//         Rectangle { width: size, length: size }
//     }
// }
// fn calculate_area() {
//     let square = Rectangle::square(10);

//     let rect = Rectangle {
//         width: 30,
//         length: 50,
//     };

//     let rect2 = Rectangle {
//         width: 10,
//         length: 40,
//     };

//     println!("{}", rect.area());

//     println!("{}", rect.can_hold(&rect2));

//     println!("{:#?}", rect);

//     println!("{:#?}", square);
// }

// let s = String::from("hello world");

// let word = first_word(&s[..]);

// println!("{}", word);
#[allow(dead_code)]
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

#[allow(dead_code)]
fn calculate_length_parent() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&mut s1);
    println!("The length of {} is {}.", s1, len);
}
fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

#[allow(dead_code)]
fn gives_ownership_parent() {
    #[allow(unused_variables)]
    let s1 = gives_ownership();
    let s2 = String::from("value");
    #[allow(unused_variables)]
    let s3 = takes_and_gives_back(s2);
}
fn gives_ownership() -> String {
    let str = String::from("Hello");
    return str;
}
fn takes_and_gives_back(str: String) -> String {
    return str;
}

#[allow(dead_code)]
fn drop_clone() {
    let s = String::from("abc");
    take_ownership(s);

    // s has moved
    // println!("{}", s);
}

fn take_ownership(str: String) {
    println!("{}", str)
}

#[allow(dead_code)]
fn string_clone() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{}, {}", s1, s2);
}

#[allow(dead_code)]
fn cycle() {
    let a = [1, 2, 3];
    for element in a.iter() {
        println!("{}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("end");

    let mut str = String::from("hello");
    str.push_str(", world!");
}
