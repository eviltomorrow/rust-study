fn main() {
    println!("Hello, world!");

    print_padovan();

    {
        let point = Box::new((0.625, 0.5));
        let label = format!("{:?}", point);
        assert_eq!("(0.625, 0.5)", label);
    }

    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: "A".to_string(),
        birth: 10,
    });
    composers.push(Person {
        name: String::from("B"),
        birth: 20,
    });
    for composer in composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    let mut s = vec!["A".to_string(), "B".to_string()];
    let t = s;
    s = vec![];
    println!("{:?}, {:?}", t, s);

    let s = vec!["A".to_string()];
    let t = s.clone();
    let u = s.clone();
    println!("{:?}, {:?}", t, u);

    let v = vec![1, 2, 3, 4];
    if is_true() {
        print_vector(v);
    }
    let v = vec![1, 2];
    println!("{:?}", v);

    let mut v = Vec::new();
    for i in 1..5 {
        v.push(i.to_string());
    }
    let c = v.remove(0);
    println!("{:?}, {:?}", v, c);
}

fn print_vector(v: Vec<i32>) {
    println!("{:?}", v);
}

fn is_true() -> bool {
    true
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}, capacity: {}", padovan, padovan.capacity());
}
