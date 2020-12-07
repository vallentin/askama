#![allow(dead_code)]

use askama::Template;
use std::collections::HashMap;

const I: i32 = 2;
const S: &str = "foo";

#[derive(Template)]
#[template(path = "implicit-borrow.html")]
struct ImplicitBorrowTemplate<'a> {
    s: &'static str,
    i: i32,
    t: ((&'static str, &'static str), (&'static str, &'static str)),
    v: Vec<i32>,
    vv: Vec<&'a [&'a str]>,
    vvv: Vec<Vec<Vec<&'a str>>>,
    m: HashMap<&'static str, i32>,
}

impl<'a> ImplicitBorrowTemplate<'a> {
    const I: i32 = 3;
    const S: &'static str = "bar";

    fn new() -> Self {
        Self {
            s: "foo",
            i: 2,
            t: (("foo", "bar"), ("baz", "qux")),
            v: vec![1, 2, 3, 4, 5],
            vv: vec![],
            vvv: vec![
                vec![vec!["1", "2"], vec!["3", "4"]],
                vec![vec!["5", "6"], vec!["7", "8"]],
            ],
            m: vec![("1", 1), ("2", 2), ("3", 3)].into_iter().collect(),
        }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn passthrough<T>(x: T) -> T {
    x
}

fn parse<S: AsRef<str>>(s: S) -> i32 {
    str::parse(s.as_ref()).unwrap()
}

#[allow(clippy::ptr_arg)]
fn vec_ref(v: &Vec<i32>) -> String {
    format!("{:?}", v)
}

fn slice_ref(v: &[i32]) -> String {
    format!("{:?}", v)
}

#[test]
fn test_implicit_borrow() {
    // Skipping testing the actual rendered output,
    // because this case is for testing if borrowing
    // is sound, as if not then the generated Rust
    // code won't compile in the first place.
    let mut t = ImplicitBorrowTemplate::new();
    let alpha = vec!["a", "b", "c"];
    let numbers = vec!["one", "two"];
    t.vv = vec![&alpha, &numbers];
    let _ = t.render().unwrap();
}
