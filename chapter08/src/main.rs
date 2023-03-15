fn main() {
    println!("Hello, world!");
    /*
    1、Vec<T> 允许我们在一个单独的数据结构中储存多于一个的值，并在内存中彼此相邻的排列所有值，只能储存相同类型的值。
    2、Vec 存放某个特定类型时，那个类型位于尖括号中。
    3、注解可能是必须，但是 vec! 宏可以推断出存储值的类型。
    4、引用 vector 中的值有两种方法，通过索引或使用 get 方法。get 方法获取的是引用。
    5、vector 动态扩容的情况下，会导致先前的引用无法使用（问题在于 vec 扩容会导致指针重新分配）
    6、for 循环中获取的 vector 引用阻止了同时对 vector 的修改。
    7、vector 默认存储的元素类型必须都相同，但是可以结合 enum 来实现不同类型的元素组合。
     */

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    println!("v is {:#?}", v);

    let v = vec![10, 20, 30, 40, 50];
    for (i, d) in v.iter().enumerate() {
        println!("i is {}, data is {}", i, d);
    }

    let third = v[2];
    let second = v.get(20);
    match second {
        Some(i) => println!("{}", i),
        None => println!("none"),
    }
    println!("third is {}", third);

    let mut v = vec![1, 2, 3, 4, 5];
    v.push(10);
    let six = &v[5];
    println!("six is {}", six);
    for i in &mut v {
        *i += 50;
    }
    println!("v is {:#?}", v);

    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let v = vec![
        Cell::Int(10),
        Cell::Float(20.0),
        Cell::Text(String::from("Hello world")),
    ];
    println!("v is {:#?}", v);

    for i in v {
        match i {
            Cell::Int(i) => println!("i is {}", i),
            Cell::Float(i) => println!("i is {}", i),
            Cell::Text(data) => println!("data is {}", data),
        }
    }
}
