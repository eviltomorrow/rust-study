use std::ops::Add;

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

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
