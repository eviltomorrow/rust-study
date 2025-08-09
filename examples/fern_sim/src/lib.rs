pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

pub fn run_simulation(fern: &mut Fern, days: usize) {
    debug_assert_eq!(1, 2);
    for _ in 0..days {
        fern.grow();
    }
}

#[test]
fn test_1() {
    println!("{}", 1 + 2);
    debug_assert_eq!(1, 2);
}

#[test]
fn test_2() {
    println!("{}", 1 + 2);
    // debug_assert_eq!(1, 2);
}
