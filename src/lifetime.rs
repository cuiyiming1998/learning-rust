pub fn get_longest() {
    let string1 = String::from("abcd");
    let string2 = "acs";

    let result = longest(string1.as_str(), string2);

    let res = get_excerpt();

    println!("{:?}", res);
    println!("{}", res.part);
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn get_excerpt() -> ImportantExcerpt<'static> {
    let novel = "abc";

    let first_sentence = novel.split(".").next().expect("Not found '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    i
}
