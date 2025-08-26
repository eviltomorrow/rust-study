pub mod exercise {
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct GrayscaleMap {
        pub pixels: Vec<u8>,
        pub size: (usize, usize),
    }

    impl GrayscaleMap {
        pub fn get_info(&self) -> String {
            let mut s = String::from(format!("{}, {} - ", self.size.0, self.size.1));
            for n in &self.pixels {
                s.push(*n as char);
            }
            s
        }
    }

    pub fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
        GrayscaleMap { pixels, size }
    }

    #[derive(Clone, Copy, Debug)]
    pub enum BroomIntent {
        FetchWater,
        DumpWater,
    }

    #[derive(Debug)]
    pub struct Broom {
        pub name: String,
        pub height: u32,
        pub health: u32,
        pub position: (f32, f32, f32),
        pub intent: BroomIntent,
    }

    pub fn chop(b: Broom) -> (Broom, Broom) {
        let mut broom1 = Broom {
            name: String::from("Hey"),
            height: 100,
            ..b
        };
        let mut broom2 = Broom {
            name: broom1.name.clone(),
            height: broom1.height / 2,
            ..broom1
        };
        broom1.name.push_str(" I");
        broom2.name.push_str(" II");

        (broom1, broom2)
    }

    #[derive(Debug)]
    pub struct Bounds(pub usize, pub usize);

    pub struct Queue<T> {
        pub older: Vec<T>,
        pub younger: Vec<T>,
    }

    impl Queue<f64> {
        pub fn sum(&self) -> f64 {
            let mut sum = 0_f64;
            for i in &self.older {
                sum += i;
            }
            for i in &self.younger {
                sum += i;
            }
            sum
        }
    }

    impl<T> Queue<T> {
        pub fn new() -> Queue<T> {
            Queue {
                older: Vec::<T>::new(),
                younger: Vec::<T>::new(),
            }
        }
        pub fn push(&mut self, t: T) {
            self.younger.push(t);
        }
        pub fn pop(&mut self) -> Option<T> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }
                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }
            self.older.pop()
        }
        pub fn is_empty(&self) -> bool {
            self.older.is_empty() && self.younger.is_empty()
        }
        pub fn split(self) -> (Vec<T>, Vec<T>) {
            (self.older, self.younger)
        }
        pub fn split_ref(&self) -> (&Vec<T>, &Vec<T>) {
            (&self.older, &self.younger)
        }
        pub fn split_box(self: Rc<Self>) {}
    }

    pub struct Vector2 {
        x: f32,
        y: f32,
    }
    impl Vector2 {
        pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
        pub const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };

        pub const ID: u32 = 10;
        pub const NAME: &str = "Vector2";

        pub fn get_info(&self) -> String {
            format!("x: {}, y: {}", self.x, self.y)
        }
    }

    pub struct Extrema<'a> {
        greatest: &'a i32,
        least: &'a i32,
    }

    impl<'a> Extrema<'a> {
        pub fn get_str(&self) -> String {
            let s = format!("{}, {}", self.greatest, self.least);
            s
        }
    }

    pub fn find_extrema<'a>(slice: &'a [i32]) -> Extrema<'a> {
        let mut greatest = &slice[0];
        let mut least = &slice[0];

        for i in 1..slice.len() {
            if slice[i] < *least {
                least = &slice[i];
            }
            if slice[i] > *greatest {
                greatest = &slice[i];
            }
        }

        Extrema { greatest, least }
    }

    pub struct Polynomial<const N: usize> {
        coefficients: [f64; N],
    }

    impl<const N: usize> Polynomial<N> {
        pub fn new(coefficients: [f64; N]) -> Polynomial<N> {
            Polynomial { coefficients }
        }

        pub fn eval(&self, x: f64) -> f64 {
            let mut sum = 0.0;
            for i in (0..N).rev() {
                sum = self.coefficients[i] + x * sum;
            }
            sum
        }

        pub fn get_str(&self) -> String {
            let mut s = String::from("Hi: ");
            for f in self.coefficients {
                s.push_str(format!("{}", f).as_str());
            }
            s
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }
}

mod extra {
    use super::exercise::Queue;

    impl<T> Queue<T> {
        pub fn with_capacity(size: usize) -> Queue<T> {
            Queue {
                older: Vec::with_capacity(size),
                younger: Vec::with_capacity(size),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::exercise::*;
    #[test]
    fn test_1() {
        let width = 1024;
        let heigt = 576;
        let image = new_map((width, heigt), vec![0; 3]);
        println!("{:?}", image.get_info());
    }

    #[test]
    fn test_2() {
        let b = Broom {
            name: String::from("Hey"),
            height: 100,
            health: 100,
            position: (0.0, 0.0, 0.0),
            intent: BroomIntent::DumpWater,
        };

        let data = chop(b);
        let (b1, b2) = data;
        println!("{:?}, {:?}", b1, b2);
    }

    #[test]
    fn test_3() {
        let b = Bounds(10, 10);
        println!("{:?}", b);
    }

    #[test]
    fn test_4() {
        let bq = Box::new(Queue::<u8>::new());
        bq.split();
    }

    #[test]
    fn test_5() {
        let bq = Queue::<char>::with_capacity(10);
        bq.split();
    }

    #[test]
    fn test_6() {
        let s = Vector2::ZERO.get_info();
        println!("{}, {}, {}", s, Vector2::ID, Vector2::NAME);
    }

    #[test]
    fn test_7() {
        let a = [0, 1, 2, 3, 4];
        find_extrema(&a);
    }

    #[test]
    fn test_8() {
        let p = Polynomial::new([1.0, 2f64, 3f64]);
        let s = p.get_str();
        println!("{}", s);
    }
}
