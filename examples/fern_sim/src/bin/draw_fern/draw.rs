#[allow(non_snake_case)]
pub fn normalFn() {
    root::c();
}

pub mod root;

#[test]
pub fn test_case1() {
    println!("this is one test case!",);

    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}
