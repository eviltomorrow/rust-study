pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    pub height: u32,
    pub width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 4,
            height: 6,
        };
        let flag = larger.can_hold(&smaller);
        assert!(flag)
    }

    #[test]
    fn smaller_can_not_hold_larger(){
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 4,
            height: 6,
        };
        let flag = smaller.can_hold(&larger);
        assert!(!flag)
    }

    #[test]
    fn assert_desc(){
        assert_eq!(2+2,4, "{}, {}", "a","b");
    }
}
