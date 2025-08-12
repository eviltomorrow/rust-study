//! 模拟蕨类职务从单个细胞开始的生长过程

// 这里定义了一个 [`Fern`] 的定义，他的方法包括 [`grow`](Fern::grow)
pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    /// 这里是增长方法的定义
    #[doc(alias = "route")]
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

#[doc = "模拟减数分裂产生孢子"]
pub fn run_simulation(fern: &mut Fern, days: usize) {
    debug_assert_eq!(1, 2);
    for _ in 0..days {
        fern.grow();
    }
}

#[test]
fn test_1() {
    println!("{}", 1 + 2);
    debug_assert_eq!(1, 1);
}

#[test]
fn test_2() {
    println!("{}", 1 + 2);
    // debug_assert_eq!(1, 2);
}

#[test]
// #[allow(unconditional_panic, unused_must_use)]
// #[should_panic(expected = "divide by zero!")]
fn test_3() {
    println!("{}", 2);

    // println!("{}", 2 / 0);
}

use std::ops::Range;
/// 如果两个范围重叠，返回
///
/// ```no_run
/// assert_eq!(fern_sim::overlap(0..7,3..10), true);
/// ```

///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}
#[test]
fn test_4() -> Result<(), std::num::ParseIntError> {
    i32::from_str_radix("1000", 10)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0));
    }
}
