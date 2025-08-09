#![allow(non_camel_case_types)]

mod draw;

fn main() {
    println!("draw");

    a();
    draw::normalFn();
}

#[cfg(target_os = "linux")]
#[allow(non_camel_case_types)]
pub struct git_revspec {}

pub struct git_error {}

fn a() {}
