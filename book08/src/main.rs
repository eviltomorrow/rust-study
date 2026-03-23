fn main() {
    println!("Hello, world!");

    use crate::mod_1_exercise;
    use crate::mod_1_exercise::mod_2_exercise::mod_3_exercise;
    mod_1_exercise::echo_hello();
    mod_3_exercise::echo_world();

    use spores;
    spores::echo_hello_1();
    spores::echo_hello_2();

    let mut s1 = 10;
    let mut s2 = 20;

    if s1 < s2 {
        std::mem::swap(&mut s1, &mut s2);
    }
    println!("s1: {}, s2: {}", s1, s2);

    use std::mem;
    if s1 < s2 {
        mem::swap(&mut s1, &mut s2);
    }

    use self::mod_1_exercise as c;
    c::echo_hello();
}

pub fn echo_hello() {
    println!("{}", "hello world");
}

mod mod_1_exercise {
    pub fn echo_hello() {
        println!("hello world");

        use mod_2_exercise;
        mod_2_exercise::echo_hello();
    }

    pub mod mod_2_exercise {
        pub mod mod_3_exercise {
            pub(in crate::mod_1_exercise::mod_2_exercise) fn echo_hello() {
                println!("hello world3")
            }

            pub(crate) fn echo_world() {
                echo_hello();
            }
        }

        pub(super) fn echo_hello() {
            println!("hello world2");

            use mod_3_exercise;
            mod_3_exercise::echo_hello();
        }
    }
}

mod child;
pub mod plant_structures;
mod spores;

#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

#[test]
fn math_works_2() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero_error() {
    1 / 0;
}

#[test]
#[should_panic(expected = "should panic")]
fn test_should_panic() {
    panic!("should panic")
}

#[cfg(test)]
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0))
    }
}
