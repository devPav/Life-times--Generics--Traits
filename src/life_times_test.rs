use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Объявление! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_retur_part(&self, announcement: &str) -> &str {
        println!("Please attension: {}", announcement);
        self.part
    }
}

pub fn lifetimes() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Jonh. A few moments later...");
    let first_sentence = novel.split(".").next().expect("Could not find '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have static life time";
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn always_first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
