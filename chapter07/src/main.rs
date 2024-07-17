use garden::vegetables::Asparagus;

mod garden;

fn main() {
    let g = garden::Garden {
        name: String::from("my garden"),
    };
    println!("{:?}, {1}", g, g.name);

    let a = Asparagus {};
    println!("{:#?}", a);
}
