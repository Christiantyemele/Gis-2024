#[allow(unused)] // not important for code
struct Pair<T> {
    value1: T,
    value2: T,
}

#[allow(unused)]
impl<T> Pair<T> {
    fn new(value1: T, value2: T) -> Self {
        Self { value1, value2 }
    }

    // swap consumes pair and returns a new pair
    fn swap(mut self) -> Self {
        let value1 = self.value2;
        let value2 = self.value1;
        Self { value1, value2 }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::Pair;

    #[test]
    fn test_generic_pair() {
        // pair for i32
        let pair_i32 = Pair::<i32>::new(5, 6);
        let new_pair = pair_i32.swap();
        assert_eq!(new_pair.value1, 6);

        // pair for string
        let pair_string = Pair::<String>::new("christian".to_owned(), "yemele".to_owned());
        let new_pair = pair_string.swap();
        assert_eq!(new_pair.value1, "yemele")
    }
}
