fn main() {
    println!("Hello, world!");

    let number_list = vec![10, 2, 19, 52, 3, 1, 17, 5];
    let mut large_number = number_list[0];
    for number in number_list {
        if large_number < number {
            large_number = number;
        }
    }
    println!("{large_number}",);

    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![5, 4, 3, 2, 1];
    largest(&v1);
    largest(&v2);

    let a = [1, 2, 3, 4, 5];
    let b = &a[1..2];
    println!("b is {:?}", b);
}

fn largest(list: &[i32]) -> i32 {
    let mut large_number = list[0];
    for number in list {
        if large_number < *number {
            large_number = *number;
        }
    }
    large_number
}
