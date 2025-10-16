#![feature(reborrow)]
use std::ops::Reborrow;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
struct CustomMarker<'a>(PhantomData<&'a ()>);
impl<'a> Reborrow for CustomMarker<'a> {}

fn method<'a>(_a: CustomMarker<'a>) -> &'a () {
    &()
}

fn main() {
    let a = CustomMarker(PhantomData);
    let b = method(a);
    let c = method(a); //~ERROR use of moved value: `a`
    println!("{c:?} {b:?} {a:?}");
}

// fn main_using_normal_references() {
//     let a = &mut ();
//     let b = method(a);
//     let _ = method(a);
//     eprintln!("{b}");
// }
