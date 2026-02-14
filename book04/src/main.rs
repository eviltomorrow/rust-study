use std::rc::Rc;

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

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    println!("{:?}", v);

    let v = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    let mut composers = Vec::new();
    composers.push(Child {
        name: Some("A".to_string()),
        birth: 10,
    });
    composers.push(Child {
        name: Some("B".to_string()),
        birth: 20,
    });
    let first_name = std::mem::replace(&mut composers[0].name, None);
    println!("{:?}", first_name);
    println!("{:?}", composers[0].birth);

    let first_name = composers[0].name.take();
    println!("{:?}", first_name);

    let l = Label { number: 10 };
    print_label(l);
    println!("My label number ls: {}", l.number);

    let s = Rc::new("A".to_string());
    let t = s.clone();
    println!("{:?}, {:?}", s, t);
}

#[derive(Copy, Clone)]
struct Label {
    number: u32,
}
fn print_label(l: Label) {
    println!("STAMP: {}", l.number);
}

struct Child {
    name: Option<String>,
    birth: i32,
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
