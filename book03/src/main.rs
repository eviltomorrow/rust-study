fn main() {
    println!("Hello, world!");

    let v = build_vector();
    println!("{:?}", v);

    let v = build_vector_2();
    println!("{:?}", v);
}

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10);
    v
}

fn build_vector_2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(20);
    v
}
