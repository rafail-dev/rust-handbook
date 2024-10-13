pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn will_cause_panic() {
    panic!("contain substring!!!");
}

#[cfg(test)]
mod tests {
    use super::*;
    // Each test is run in a new thread, and when the main thread sees
    // that a test thread has died, the test is marked as failed.
    #[test]
    fn it_works() {
        println!("this output will not be displayed");
        // or `cargo test -- --show-output`

        let result = add(2, 2);
        // ==
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    #[test]
    fn asserts() {
        // values being compared must implement the PartialEq and Debug traits

        // logic
        assert!(true);

        // ==
        assert_eq!(1, 1);

        // !=
        assert_ne!(1, 2);

        // message on fail
        assert!(true, "message text");
    }

    #[test]
    #[should_panic]
    fn panic() {
        will_cause_panic();
    }

    #[test]
    #[should_panic(expected = "substring")]
    fn panic_with_expected_message() {
        will_cause_panic();
    }

    #[test]
    fn result() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn another_result() -> Result<(), String> {
        // If this is not a number, Err will be returned immediately
        let a = "2"
            .parse::<i32>()
            .map_err(|_| String::from("Failed to parse a into i32"))?;

        let b = "2"
            .parse::<i32>()
            .map_err(|_| String::from("Failed to parse b into i32"))?;

        assert_eq!(a + b, 4);
        Ok(())
    }

    // cargo test -- --ignored
    // cargo test -- --include-ignored
    #[test]
    #[ignore]
    fn ignored() {}
}
