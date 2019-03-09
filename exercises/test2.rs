// test2.rs
// This is a test for the following sections:
// - Tests

// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests that we get the result
// we expect to get when we call `times_two` with a negative number.
// No hints, you can do this :)

pub fn times_two(num: i32) -> i32 {
    num * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_twice_of_positive_numbers() {
        assert_eq!(4, times_two(2));
        assert_eq!(10, times_two(5));
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        assert_eq!(-4, times_two(-2));
        assert_eq!(-20, times_two(-10));
    }
}
