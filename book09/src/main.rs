use std::rc::Rc;

fn main() {
    println!("Hello, world!");

    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);

    new_map((width, height), vec![0; width * height]);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::DumpWater,
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);

    println!("{:?}", hokey1.intent);
    println!("{:?}", hokey1.position);

    println!("{:?}", BroomIntent::FetchWater);

    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    let image_bounds = bounds2(10, 10);
    assert_eq!(image_bounds.0 + image_bounds.1, 20);

    let d = Ascii(vec![0; 3]);
    display(d);

    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('c');

    assert_ne!(q.is_empty(), true);

    let mut bq = Box::new(Queue::new());
    bq.push('c');
    println!("{}", bq.len());

    let n = Node {
        children: Vec::new(),
    };
    let mut parent = Node {
        children: Vec::new(),
    };
    let c = Rc::new(n);
    c.append_to(&mut parent);

    let scaled = Vector2::UNIT.scaled_by(2.0);
    println!("{}", scaled);

    Vector2::ZERO.print();
    println!("{}, {}", Vector2::ID, Vector2::NAME);

    let mut q = Queue::<f64>::new();
    q.push(10.0);
    let sum = q.sum();
    println!("{}", sum);

    let mut slice = Vec::<i32>::new();
    slice.push(1);
    slice.push(2);
    slice.push(3);
    let e = find_extrema(&slice);
    println!("{:?}, {}, {}", e, e.greatest, e.least);

    let p = Polynomial::new([1.0, 2.0, 3.0]);
    let sum = p.eval(10.0);
    println!("{}", sum);
}

fn bounds2(e0: usize, e1: usize) -> Bounds2 {
    Bounds2(e0, e1)
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Clone, Copy, Debug)]
enum BroomIntent {
    FetchWater,
    DumpWater,
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

struct Bounds(usize, usize);

pub struct Bounds2(pub usize, pub usize);

struct Ascii(Vec<u8>);

fn display(data: Ascii) {
    for u in data.0 {
        println!("{}", u);
    }
}

pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl Queue<f64> {
    fn sum(&self) -> f32 {
        let mut sum = 0f64;
        for c in &self.older {
            sum += c;
        }
        for c in &self.younger {
            sum += c;
        }
        sum as f32
    }
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
}

impl<T> Queue<T> {
    fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
    fn len(&self) -> usize {
        self.older.len() + self.younger.len()
    }
}

impl<T> Queue<T> {
    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
}

struct Node {
    children: Vec<Rc<Node>>,
}

impl Node {
    fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
}

pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 1.0 };

    const ID: u32 = 18;
    const NAME: &'static str = "Vector2";
}

impl Vector2 {
    fn scaled_by(&self, val: f32) -> f32 {
        self.x + self.y + val
    }
    fn print(&self) {
        println!("{}, {}", self.x, self.y);
    }
}

#[derive(Debug)]
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greater = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i]
        }
        if slice[i] > *greater {
            greater = &slice[i]
        }
    }
    Extrema {
        greatest: greater,
        least: least,
    }
}

struct Polynomial<const N: usize> {
    coefficients: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Self {
        Polynomial { coefficients }
    }

    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }
        sum
    }
}
