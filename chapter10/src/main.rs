fn main() {
    println!("Hello, world!");

    let v1 = vec![1, 2, 3, 4, 5, 6];
    let v2 = vec![6, 5, 4, 3, 2, 1];
    println!("{}", large(&v1));
    println!("{}", large(&v2));

    let v1 = vec![1, 2, 3, 4, 5, 6, 7];
    println!("{}", large_i32(&v1));

    let v2 = vec!['c', 'b', 'a', 'e', 'f'];
    println!("{}", large_char(&v2));

    let v3 = vec![1,2,3,4,5,6];
    println!("{}", largest(&v3));
}

fn large(list: &[i32]) -> &i32 {
    let mut largest_number = &list[0];
    for number in list {
        if largest_number < number {
            largest_number = number;
        }
    }
    largest_number
}

fn large_i32(list: &[i32]) -> &i32 {
    let mut largest_number = &list[0];
    for number in list {
        if largest_number < number {
            largest_number = number;
        }
    }
    largest_number
}

fn large_char(list: &[char]) -> &char {
    let mut larget_char = &list[0];
    for number in list {
        if larget_char < number {
            larget_char = number;
        }
    }
    larget_char
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest_value = &list[0];
    for value in list {
        if largest_value < value {
            largest_value = value
        }
    }
    largest_value
}
