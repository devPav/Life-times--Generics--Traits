use std::{fmt::{Display, Debug}};

use crate::traits_test::Summary;

mod traits_test;
mod life_times_test;

struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    // Functions
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    // Generic functions
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest char is {}", result);

    // Generic structures, enums
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("char_list.x = {}", integer.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point {
        x: "Привет",
        y: 'c',
    };
    println!("{:?}{:?}", p1.x, p2.x);
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Traits
    let tweet = traits_test::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("конечно, как вы, наверное, уже знаете, люди"),
        reply: false,
        retweet: false,
    };
    println!("1 New tweet: {}", tweet.summarize());

    let article = traits_test::NewsArticle {
        headline: String::from("Пингвины выигрывают Кубок Стэнли!"),
        location: String::from("Питтсбург, шт. Пенсильвания, США"),
        author: String::from("Айсбург"),
        content: String::from("Питтсбург Пингвинз снова является лучшей хоккейной командой в НХЛ."),    
    };
    println!("Есть новая статья! {}", article.summarize());

    // Reference life times. 247
    life_times_test::lifetimes();
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
pub fn notify(item: impl Summary + Display) -> impl Summary {
    println!("Urgent news! {}", item.summarize());
    item
}
pub fn notify2<T: Summary + Display>(item: T) {
    println!("Urgent news! {}", item.summarize());
}
pub fn notiy3<T, U>(item: T, item2: U) -> i32 
    where T:Summary + Clone, U: Debug + Copy {
    32    
}
