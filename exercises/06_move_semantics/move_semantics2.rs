fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.clone();
    new_vec.push(88);
    new_vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let mut vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(&mut vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}