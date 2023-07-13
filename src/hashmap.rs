use std::collections::HashMap;

pub fn hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("Score is {}.", s),
        None => println!("Team not exist."),
    }

    for (k, v) in &scores {
      println!("{}, {}", k, v);
    }

    let times = count_word_times("hello world wonderful world");

    // let map = create_from_tuple();
    println!("{:?}", times);
}

// fn create_from_tuple() -> HashMap<&&str, &i32> {
//     let teams = vec!["value", "v"];
//     let initial_scores = vec![10, 50];

//     let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

//     scores
// }

fn count_word_times(text: &str) -> HashMap<&str, i32> {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
    }

    map
}