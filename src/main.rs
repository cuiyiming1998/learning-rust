fn main() {
    calculate_length_parent();
}

fn calculate_length_parent() {
    let s1 = String::from("v");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {
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
    return str
}
fn takes_and_gives_back(str: String) -> String {
    return str
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