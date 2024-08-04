pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(s: i32) -> i32 {
    s + 2
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be 1 and 100, go {}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 8,
        };
        let smaller = Rectangle {
            width: 5,
            height: 3,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting not contain name: {}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore = "reason"]
    fn it_works3() {
        if 2 + 3 == 4 {
            panic!("wrong");
        }
    }
}
