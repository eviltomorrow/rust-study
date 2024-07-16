fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}, {:#?}", four, six);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
