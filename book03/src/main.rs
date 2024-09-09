fn main() {
    println!("Hello, world!");

    let x = "1";
    assert_eq!(x.parse::<i32>().unwrap(), 1);

    let f: Foo = foobar(10);
    assert_eq!(f, Foo(10));

    let b: Bar = foobar(20);
    assert_eq!(b, Bar(20, 30));

    let c: Foo = foobar2(10);
    println!("{:?}", c);
}

fn foobar<T: Inst>(i: i32) -> T {
    T::new(i)
}

fn foobar2<T>(i: i32) -> T
where
    T: Inst,
{
    T::new(i)
}

trait Inst {
    fn new(i: i32) -> Self;
}

#[derive(Debug, PartialEq)]
struct Foo(i32);
impl Inst for Foo {
    fn new(i: i32) -> Foo {
        Foo(i)
    }
}

#[derive(Debug, PartialEq)]
struct Bar(i32, i32);
impl Inst for Bar {
    fn new(i: i32) -> Bar {
        Bar(i, i + 10)
    }
}
