use std::any::Any;

pub trait Object {
    fn type_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub fn type_name(x: &dyn Object) -> &str {
    x.type_name()
}

pub fn is_of_type<T: 'static>(x: &dyn Object) -> bool {
    x.as_any().is::<T>()
}

impl Object for i32 {
    fn type_name(&self) -> &str {"i32"}
    fn as_any(&self) -> &dyn Any {self}
}

// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder<T>(x: T) {
    if type_of(x) == &str {

    }
}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder("nice weather".into());

    placeholder(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
