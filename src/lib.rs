// mod front_of_house {
//   pub mod hosting {
//     pub fn add_to_waitlist() {}
//   }

//   mod serving {

//   }
// }

// pub fn eat_at_restaurant() {
//   crate::front_of_house::hosting::add_to_waitlist();
// }

// ------------------

// fn serve_order() {}

// mod back_of_house {

//   fn fix_incorrect_order() {
//     cook_order();
//     // crate::serve_order();
//   }

//   fn cook_order() {}
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//   let mut meal = back_of_house::Breakfast::summer("Rye");
//   meal.toast = String::from("Wheat");

//   println!("{}", meal.toast);
// }

// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }
