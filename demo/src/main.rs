fn main() {
    println!("Hello, world!");

    let mut c = vec![1, 2, 3, 4, 5];

    change_vec(&mut c);
    println!("{:?}", c);
}

fn change_vec(v: &mut Vec<i32>) {
    let n1 = v[1];
    v[0] = n1;
}
