use std::rc::Rc;

fn main() {
    println!("Hello, world!");

    print_padovan();

    {
        let point = Box::new((0.625, 0.5));
        let label = format!("{:?}", point);
        println!("{}", label);
    }

    let v = vec![String::from("A")];
    let x = &v[0];
    println!("{}", x);

    let mut composers = Vec::<Person>::new();
    composers.push(Person {
        name: Some("Liarsa".to_string()),
    });

    println!("{:?}", composers);

    let first_name_option = std::mem::replace(&mut composers[0].name, None);
    match first_name_option {
        Some(data) => println!("{}", data),
        None => println!("None"),
    }
    println!("{:?}", composers);

    composers.push(Person {
        name: Some("Liarsa".to_string()),
    });
    let first_name_option = composers[1].name.take();
    match first_name_option {
        Some(data) => println!("{}", data),
        None => println!("None"),
    }
    println!("{:?}", composers);

    let a = Animal { age: 10 };
    println!("{:?}", a);
    print_animal(a);
    println!("{:?}", a);

    let s = Rc::new(String::from("Hello world"));
    println!("{}", s);
    let t = s.clone();

    println!("{}, {}", s, t);
}

fn print_animal(a: Animal) {
    println!("{:?}, {}", a, a.age);
}

#[derive(Debug, Clone, Copy)]
struct Animal {
    age: u32,
}

#[derive(Debug)]
struct Person {
    name: Option<String>,
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}
