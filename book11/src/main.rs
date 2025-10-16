use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{self, Write},
    ops::Range,
    vec,
};

type GenericError = Box<dyn std::error::Error>;

fn main() -> Result<(), GenericError> {
    let mut bytes = vec![];
    let r = say_hello(&mut bytes);
    match r {
        Ok(_) => println!("ok"),
        Err(err) => println!("{}", err),
    }
    println!("{:?}", bytes);

    let mut local_file = File::create("hello.txt")?;
    let _ = say_hello(&mut local_file)?;

    let mut buf = vec![];
    let size = buf.write(b"Hello world")?;
    println!("{:?}, {}", buf, size);

    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;
    writer.write(b"Hello world!")?;

    let mut w: Box<&mut dyn Write> = Box::new(&mut local_file);
    w.write(b"Hi")?;

    say_hello2(&mut local_file)?;
    say_hello2::<File>(&mut local_file)?;
    say_hello3::<File>(&mut local_file)?;

    top_ten1::<Vec<i32>>(&vec![]);
    top_ten2::<HashMap<String, i32>>(&HashMap::new());

    let v1 = (0..10).collect::<Vec<i128>>();
    println!("{:?}", v1);

    let sum = dot_produce([1.0, 2., 3f64], [1.0, 2.0, 3.0]);
    println!("{:?}", sum);

    let sum = dot_produce::<3>([0.0; 3], [1.0; 3]);
    println!("{:?}", sum);

    let t = Tomato {};
    let p = Potato {};

    let s = Salad {
        veggies: vec![Box::new(t), Box::new(p)],
    };
    for v in s.veggies {
        v.eat();
    }

    let v: &dyn Visible = &Broom {
        x: 10,
        y: 10,
        height: 2,
    };
    v.draw(&mut Canvas {});
    v.hit_test(10, 10);

    let mut out = Slink;
    out.write_all(b"Hello world\r\n")?;

    assert_eq!('$'.is_emoji(), true);

    let mut v: Vec<u8> = vec![];
    if let Err(err) = v.write_html(&HtmlDocument {}) {
        println!("{:?}", err);
    }

    let w = Warrier {};
    let c: &dyn Creature = &w;
    c.position();

    "hello".to_string();
    str::to_string("hello");
    ToString::to_string("hello");
    <str as ToString>::to_string("hello");

    c.position();
    Creature::position(c);

    Ok(())
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\r\n")?;
    Ok(())
}

fn say_hello2<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\r\n")?;
    Ok(())
}

fn say_hello3<W>(out: &mut W) -> std::io::Result<()>
where
    W: Write,
{
    out.write_all(b"say_hello3")?;
    Ok(())
}

fn top_ten1<T: Debug + Eq>(_: &T) {}

fn top_ten2<T>(_: &T)
where
    T: Debug,
    T: Eq,
    T: Debug + Eq,
{
}

fn dot_produce<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }
    sum
}

#[allow(dead_code)]
struct PancakeStack {}

#[allow(dead_code)]
impl PancakeStack {
    fn push<T: Eq>(t: T) {
        let _ = t;
    }
}

#[allow(dead_code)]
type PancakeResult<T> = Result<T, dyn std::error::Error>;

trait Vegetable {
    fn eat(&self);
}

struct Salad {
    veggies: Vec<Box<dyn Vegetable>>,
}

#[allow(dead_code)]
struct Salad2<'a> {
    veggies: Vec<&'a dyn Vegetable>,
}

struct Tomato {}

impl Vegetable for Tomato {
    fn eat(&self) {
        println!("I can eat tomato");
    }
}

struct Potato {}

impl Vegetable for Potato {
    fn eat(&self) {
        println!("I can eat potato");
    }
}

struct Canvas {}

impl Canvas {
    fn write_at(&self, x: i32, y: i32, c: char) {
        println!("{}, {}, {}", x, y, c);
    }
}

trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

struct Broom {
    x: i32,
    y: i32,
    height: i32,
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1..self.y
    }
}

pub struct Slink;

impl Write for Slink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

trait Emoji {
    fn is_emoji(&self) -> bool;
}

impl Emoji for char {
    fn is_emoji(&self) -> bool {
        return true;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct HtmlDocument {}

#[allow(dead_code)]
trait WritenHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

impl<W: Write> WritenHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        println!("{:?}", html);
        Err(io::Error::from(io::ErrorKind::Other))
    }
}

pub trait Spliceable {
    fn splice<'b>(&self, other: &'b Self) -> &'b Self;
}

impl Spliceable for HtmlDocument {
    fn splice<'b>(&self, other: &'b Self) -> &'b Self {
        other
    }
}

trait Creature: Visible {
    fn position(&self) -> (i32, i32);
}

struct Warrier {}

impl Creature for Warrier {
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }
}

impl Visible for Warrier {
    fn draw(&self, canvas: &mut Canvas) {
        let _ = canvas;
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        x > y
    }
}

#[allow(dead_code)]
trait Iterator2 {
    type Item;
    fn next();
}

#[allow(dead_code)]
struct Args {}

impl Iterator2 for Args {
    type Item = String;

    fn next() {
        todo!()
    }
}
