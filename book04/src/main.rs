fn main() {
    print_padovan();

    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");

    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: String::from("Palestrina"),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    for p in composers {
        println!("{}, {}", p.name, p.birth);
    }

    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    let u = t;
    println!("{:?}", u);

    let x = vec![10, 20, 30];
    let flag = ok();
    if flag {
        move_vec(&x);
    } else {
        borrow_vec(&x);
    }
    println!("{:?}", x);

    let mut x = Vec::<String>::new();
    x.push("AA".to_string());
    x.push("BB".to_string());
    x.push("CC".to_string());

    for v in x {
        println!("{:?}", v);
    }

    move_string(String::from("Hello"));

    let x = String::from("Hello");
    if x == "Hello" {
        println!("true");
    }
    println!("{}", x);

    let x = String::from("Hello");
    let y = String::from("Hello");
    println!("{}", x == y);

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    println!("{:?}", v);

    let fifth = v.pop().expect("vector empty");
    assert_eq!(fifth, "105");
    println!("{:?}", v);

    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    println!("{:?}", v);

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    println!("{:?}", v);

    #[derive(Debug)]
    struct Animal {
        name: Option<String>,
    }

    let mut composers = Vec::new();
    composers.push(Animal {
        name: Some("Dog".to_string()),
    });
    let first_name = std::mem::replace(&mut composers[0].name, None);
    println!("{:?}, {:?}", first_name, composers);
    assert_eq!(composers[0].name, None);

    let first_name = composers[0].name.take();
    println!("{:?}", first_name);

    #[derive(Clone, Copy)]
    struct Lable {
        number: u32,
    }

    fn print(l: Lable) {
        println!("STAMP: {}", l.number);
    }

    let l = Lable { number: 3 };
    print(l);
    println!("My label number is: {}", l.number);
}

fn move_vec(x: &Vec<i32>) {
    println!("{:?}", x);
}

fn borrow_vec(x: &Vec<i32>) {
    println!("{:?}", x);
}

fn move_string(x: String) {
    println!("{}", x);
}

fn ok() -> bool {
    false
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];

    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("{:?}", padovan);
}
