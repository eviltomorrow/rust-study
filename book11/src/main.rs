use std::io;
use std::iter;
use std::vec::IntoIter;
use std::{fmt::Debug, fs::File, hash::Hash, io::Write, ops::Range};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn main() -> GenericResult<()> {
    println!("Hello, world!");

    let mut local_file = File::create("hello.txt")?;
    say_hello1(&mut local_file)?;

    let mut bytes = vec![];
    say_hello2(&mut bytes)?;
    println!("{:?}", String::from_utf8(bytes));

    let a: [f64; 3] = [0., 0., 0.];
    let b = [1f64; 3];
    let sum = dot_product(a, b);
    println!("{:?}", sum);

    let _s = Salad3 {
        veggies: vec![&Tomato {}, &Potato {}],
    };

    let _s = Salad4 {
        veggies: vec![Box::new(Tomato {}), Box::new(Potato {})],
    };

    let mut c = Canvas {};
    let v: &dyn Visible = &Broom {
        x: 10,
        y: 10,
        height: 20,
    };
    v.draw(&mut c);
    v.hit_test(10, 20);

    let v: &dyn Visible = &NonBroom {};
    v.draw(&mut c);

    let mut s = Sink {};
    let b = b"hello world";
    if let Ok(data) = s.write_all(b) {
        println!("{:?}", data);
    }

    let c = 'c';
    println!("{}", c.is_emoji());

    let mut c = Write1 {};
    let _ = c.write_html(&HtmlDocument {});

    let s = CherryTree { num: &10 };
    s.splice(&s);

    let m = Mammoth {};
    m.splice(&m);

    let s1 = SortedStringSet::from_slice(&vec![""]);
    println!("{}", s1.contains(""));

    unknown_words(&vec![String::from("H")], &s1);
    unknown_words_2(&vec![String::from("H")], &s1);
    Ok(())
}

fn say_hello1(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(br"hello world\n")?;
    out.flush()
}

fn say_hello2<T: Write>(out: &mut T) -> std::io::Result<()> {
    out.write_all(br"hello world\n")?;
    out.flush()
}

fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] + b[i];
    }
    sum
}

#[allow(dead_code)]
fn top_ten<T: Debug>(_values: &Vec<T>) {}

#[allow(dead_code)]
fn top_ten_1<T: Debug + Hash + Eq>(_values: &Vec<T>) {}

#[allow(dead_code)]
fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(_data_1: M, _data_2: R) {}

#[allow(dead_code)]
fn run_query_1<M, R>()
where
    M: Mapper + Serialize,
    R: Reducer + Serialize,
{
}

#[allow(dead_code)]
trait Mapper {}
#[allow(dead_code)]
trait Reducer {}
#[allow(dead_code)]
trait Serialize {}

#[allow(dead_code)]
fn nearest<'t, 'c, P>(_target: &'t P, candidates: &'c [P]) -> &'c P
where
    P: Debug,
{
    &candidates[0]
}

#[allow(dead_code)]
enum C<T>
where
    T: Debug,
{
    NUM3(T),
}

#[allow(dead_code)]
trait Vegetable {
    fn eat(&self);
}

struct Tomato {}

impl Vegetable for Tomato {
    fn eat(&self) {
        println!("very good!");
    }
}

struct Potato {}

impl Vegetable for Potato {
    fn eat(&self) {
        println!("not good!");
    }
}

#[allow(dead_code)]
struct Salad<V: Vegetable> {
    veggies: Vec<V>,
}

#[allow(dead_code)]
struct Salad2<'a, V: Vegetable> {
    veggies: Vec<&'a V>,
}

#[allow(dead_code)]
struct Salad3<'a> {
    veggies: Vec<&'a dyn Vegetable>,
}

#[allow(dead_code)]
struct Salad4 {
    veggies: Vec<Box<dyn Vegetable>>,
}

trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

struct Canvas {}

impl Canvas {
    fn write_at(&self, _x: i32, _y: i32, _c: char) {}
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

struct Broom {
    x: i32,
    y: i32,
    height: i32,
}

impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1..self.y
    }
}

struct NonBroom {}

impl Visible for NonBroom {
    fn draw(&self, canvas: &mut Canvas) {
        let _ = canvas;
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        let _ = x;
        let _ = y;
        true
    }
}

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        false
    }
}

trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

struct HtmlDocument {}

impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        let _ = html;
        println!("Here");
        Ok(())
    }
}

struct Write1 {}

impl Write for Write1 {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}

struct CherryTree<'a> {
    num: &'a i32,
}

impl<'a> Spliceable for CherryTree<'a> {
    fn splice(&self, other: &Self) -> Self {
        let _ = other;
        Self { num: other.num }
    }
}

struct Mammoth {}

impl Spliceable for Mammoth {
    fn splice(&self, other: &Self) -> Self {
        let _ = other;
        Self {}
    }
}

#[allow(dead_code)]
trait MegaSpliceable {
    fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable>;
}

trait StringSet {
    fn new() -> Self
    where
        Self: Sized;
    fn from_slice(strings: &[&str]) -> Self
    where
        Self: Sized;
    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str);
}

struct SortedStringSet {}

impl StringSet for SortedStringSet {
    fn new() -> Self {
        SortedStringSet {}
    }

    fn from_slice(strings: &[&str]) -> Self {
        let _ = strings;
        SortedStringSet {}
    }

    fn contains(&self, string: &str) -> bool {
        let _ = string;
        true
    }

    fn add(&mut self, string: &str) {
        let _ = string;
    }
}

fn unknown_words<S: StringSet>(document: &[String], wordlist: &S) -> S {
    let mut unknowns = S::new();
    for word in document {
        if wordlist.contains(word) {
            unknowns.add(word);
        }
    }
    unknowns
}

fn unknown_words_2(document: &[String], wordlist: &dyn StringSet) -> Box<dyn StringSet> {
    let _ = document;
    let _ = wordlist;
    Box::new(SortedStringSet {})
}

pub trait Iterator2 {
    type Item;
    type C;
    fn next(&mut self) -> Option<Self::C>;
}

impl Iterator2 for std::env::Args {
    type Item = String;
    type C = i32;
    fn next(&mut self) -> Option<i32> {
        todo!()
    }
}

#[allow(dead_code)]
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

#[allow(dead_code)]
fn dump<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    for (index, value) in iter.enumerate() {
        println!("{}, {:?}", index, value);
    }
}

#[allow(dead_code)]
fn dump_2<I>(iter: I)
where
    I: Iterator<Item = String>,
{
    for (index, value) in iter.enumerate() {
        println!("{}, {:?}", index, value);
    }
}

#[allow(dead_code)]
fn dump_3(iter: &mut dyn Iterator<Item = String>) {
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value)
    }
}

#[allow(dead_code)]
fn cyclical_zip_1(v: Vec<u8>, u: Vec<u8>) -> iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

#[allow(dead_code)]
fn cyclical_zip_2(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
    Box::new(v.into_iter().chain(u.into_iter()).cycle())
}

#[allow(dead_code)]
fn cyclical_zip_3(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}
