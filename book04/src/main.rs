use std::rc::Rc;

fn main() {
    println!("Hello, world!");

    print_padovan();

    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");

    let mut composers = Vec::new();
    composers.push(Person {
        name: "shepard".to_string(),
        birth: 20,
    });

    composers.push(Person {
        name: "captain".to_owned(),
        birth: 10,
    });

    composers.push(Person {
        name: "liarsa".to_string(),
        birth: 30,
    });

    for composer in composers {
        println!("name: {}, birth: {}", composer.name, composer.birth);
    }

    let s = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let t = s;
    let v = t;
    println!("{:?}", v);

    let mut s = "Govinda".to_string();
    let t = s;
    println!("{}", t);
    s = "Siddhartha".to_string();
    println!("{}", s);

    // let x = vec![10, 20, 30];
    // let flag = false;
    // if flag {
    //     let d = x;
    //     println!("{:?}", d);
    // }
    // println!("{:?}", x);

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    let third = v[2].clone();
    println!("{}, {:?}", third, v);

    if v[2] == "103".to_string() {
        println!("{}", v[2]);
    }

    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    println!("{:?}", v);

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    println!("{}, {:?}", third, v);

    let v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    let s1 = Person2 {
        name: Some("hello".to_string()),
        birth: 20,
    };
    let s2 = Person2 {
        name: Some("sheard".to_string()),
        birth: 10,
    };

    let mut v = vec![s1, s2];
    let first_name = std::mem::replace(&mut v[0].name, None);
    println!("{:?}, {}, {:?}", first_name, v[0].birth, v);

    let second_name = v[1].name.take();
    println!("{:?}, {}, {:?}", second_name, v[1].birth, v);

    let string1 = "somnambulance".to_string();
    let string2 = string1;
    println!("{}", string2);

    let num1: i32 = 36;
    let num2 = num1;
    println!("{}, {}", num1, num2);

    #[derive(Clone, Copy)]
    struct Label {
        number: i32,
    }

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let l = Label { number: 10 };
    print(l);
    println!("My Label number is: {}", l.number);

    let s: Rc<String> = Rc::<String>::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = t.clone();
    assert!(s.contains("hira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
}

#[derive(Debug)]
struct Person2 {
    name: Option<String>,
    birth: i32,
}

struct Person {
    name: String,
    birth: i32,
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        padovan.push(padovan[i - 3] + padovan[i - 2]);
    }
    println!("{:?}", padovan);
}
