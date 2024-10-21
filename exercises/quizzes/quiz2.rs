// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

#[derive(PartialEq)]
enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use std::collections::btree_map::Values;

    use super::Command;

    // TODO: Complete the function as described above.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String>{ 
        let mut res_vec = Vec::new();
        for (a, b) in input {
            match b {
                Command::Uppercase => res_vec.push(a.to_uppercase()),
                Command::Trim => res_vec.push(String::from(a.trim())),
                Command::Append(times) => {
                    let mut clone = a.clone();
                    for _ in 0..times {
                        clone.push_str("bar")

                    } 
                    res_vec.push(clone)
                }
            }
        }
        return res_vec
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = my_module::transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
