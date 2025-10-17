use std::{ops::Add, process::Output};

fn main() {
    let a = Complex { re: 10, im: 10 };
    let b = Complex { re: 20, im: 20 };
    let c = a + b;
    println!("{:?}", c);
}

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

impl Add for Complex<i32> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Add for Complex<T>
where :
    Add<Output=T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {

    }
}
