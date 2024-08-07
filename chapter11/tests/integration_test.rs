use chapter11;

mod common;
#[test]
fn it_works() {
    common::setup();
    let result = chapter11::add(2, 2);
    assert_eq!(result, 4);
}
