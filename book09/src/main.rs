fn main() {
    println!("Hello, world!");

    let width = 1024;
    let height = 576;

    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    println!("{:?}", image.size);

    let g = new_grayscale_map(vec![0; width * height], (width, height));
    println!("{:?}", g.size);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200f32, 0.0),
        intent: BroomIntent::DumpWater,
    };

    let (hokey1, hokey2) = chop(hokey);
    println!("{:?}, {:?}", hokey1, hokey2);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

#[allow(dead_code)]
#[derive(Debug)]
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn new_grayscale_map(pixels: Vec<u8>, size: (usize, usize)) -> GrayscaleMap {
    GrayscaleMap { pixels, size }
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };

    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}
