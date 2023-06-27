#[allow(dead_code)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[allow(dead_code)]
pub fn vec() {
    // let v: Vec<i32> = Vec::new();
    #[allow(unused_mut)]
    let mut v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        None => println!("no third element"),
        Some(third) => println!("The third element is {}", third),
    }

    // for i in &mut v {
    //     *i += 50
    // }

    // for i in v {
    //     println!("{}", i);
    // }

    #[allow(unused_variables)]
    let row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(1.1),
        SpreadsheetCell::Text(String::from("hello")),
    ];
}
