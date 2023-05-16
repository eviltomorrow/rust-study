fn main() {
    println!("Hello, world!");
    println!("Hello");
    /*
    所有权的规则
    1. Rust 中的每一个值都有一个所有者（owner）。
    2. 值在任一时刻有且只有一个所有者。
    3. 当所有者（变量）离开作用域，这个值将被丢弃。

    做用域是一个项（item）在程序中有效的范围。当变量离开作用域时 }，会调用一个 drop 函数，释放内存
    4. 字符串字面值被硬编码在代码中。所以是不可变的，高效。
    5、变量从声明的地方开始到当前作用域结束都是有效的。
    */

    let mut s = String::new();
    s.push_str("hello");
    println!("s is {}", s);

    let x = 5;
    let y = x; // 此时，栈中含有两个值 x、y, 并且都是 5（复制）
    println!("y is {}", y);

    let s1 = String::from("hello");
    let s2 = s1; // 对于堆中的变量，s1 的所有权移动到了 s2, s1 被标记为无效（浅拷贝）
    println!("s2 is {}", s2);

    let s1 = String::from("world");
    let s2 = s1.clone(); // 深度拷贝，堆内含有两份数据
    println!("s1 is {}, s2 is{}", s1, s2);

    /*
    1、拷贝：栈上的数据
    2、移动：堆上的数据
    3、自定义类型如果实现了 Copy trait（实现了 Drop trait 的类型不能实现 Copy trait），那么旧变量在赋值给其他变量后仍然可用。
    4、规则：任何简单的标量值的组合都可以实现 Copy
    5、将值传递给函数与变量赋值的原理相似。向函数传递值可能会移动或者复制，就像赋值语句一样。
    6、将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
     */
    let s = give_ownership();
    println!("s is {}", s);

    /*
    引用（refrence）像一个指针，因为它是一个地址，我们可以由此访问存储于该地址的属于其他变量的数据。（引用确保只想某个特定类型的有效值）
        参数 s ->  s1  -> 堆数据
    创建一个引用的行为被称为借用（borrowing）

    可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能在使用它之前，创建对该变量的引用。
    防止同一时间，对同一数据存在多个可变引用。避免数据竞争。
    1、在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
    2、引用必须总是有效的（避免悬垂引用）。
    3、注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
     */
    let s1 = String::from("hello");
    let length = claculate_length(&s1);
    println!("length is {}", length);

    let mut s = String::new();
    change_string(&mut s);
    println!("s is {}", s);

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    /*
    slice 允许你引用集合中一段连续的元素序列，而不引用整个集合。
     */
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..];
    println!("hello is {}, world is {}", hello, world);

    // 这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。
    let _s = "Hello world";

    // 这个 slice 的类型是 &[i32]
    let _a = [1, 2, 3, 4, 5];
}

fn give_ownership() -> String {
    String::from("Hello")
}

fn claculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str("hello world");
}
