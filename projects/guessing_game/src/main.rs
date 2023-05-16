use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!(
                    "Faile to parse number, nest error: {}, please input number!",
                    error
                );
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/*
   1、默认情况下，rust 设定了若干个自动导入到每个程序作用域中的标准库内容，被称为预导入(preclude)。
   2、:: 语法表明 new 是 String 类型的一个关联函数。关联汉书是针对类型实现的，而不是 String 的某个特定实例（静态方法）。
   3、隐藏（Shadowing）功能经常将一个类型的只转换为另一个类型的值。
*/
