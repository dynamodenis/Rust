mod basket;
mod stack;
mod container;

use num_traits::{ToPrimitive, Float};
use basket:: Basket;
use stack::Stack;

use container::Container;

fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64{
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn add_string<T: Container<String>>(c: &mut T, s: String){
    c.put(s);
}
fn main() {
    let a: f32 =  3.0;
    let b: f64 = 4.0;
    println!("{}", solve(a, b));

    let mut b1 = Basket::new("apple".to_string());

    let b2 = Basket::new(10);

    let mut s1 = Stack::new(vec!["apple".to_string(), "banana".to_string()]);

    add_string(&mut b1, "orange".to_string());

    add_string(&mut s1, "grape".to_string());
}
