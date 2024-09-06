use std::ops::Deref;

fn main() {
    let x = 10;
    println!("{}", x);

    let b = Box::new(10);
    println!("b = {}", b);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("{:?}, {:?}", c, d);
    let e = CustomSmartPointer2 { data: 10 };
    let f = e;
    println!("{:?}", f);
    println!("CustomSmartPointers created.");

    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("World"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

#[derive(Debug)]
struct CustomSmartPointer2 {
    data: i32,
}

impl Drop for CustomSmartPointer2 {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer2 with data `{}`", self.data);
    }
}
