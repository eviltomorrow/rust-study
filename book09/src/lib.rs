pub mod exercise {
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
}
