use std::fmt::Display;

pub fn vec_to_string<T: Display>(vec: &Vec<T>) -> String {
    vec.iter()
        .map(|el| el.to_string())
        .reduce(|acc, s| acc + " " + &s + "\n\t")
        .unwrap_or_default()
}