pub fn string() {
    let mut s = String::new();
    let data = "initial contents";

    let hello = String::from("你好");

    println!("{}", hello.len());

    for i in hello.chars() {
      println!("{}", i);
    }

    s.push_str(data);

    println!("{}", s);
}