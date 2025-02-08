// --------------------------------
// Implied bounds
// --------------------------------

struct GenericWithLifetime<'a, T> {
    b: &'a T, // implied T: 'a: 'b => 'a outlives 'b and now T: 'a => T outlives 'a
}

fn lifetime_bound_implied_fn<'a, T>(GenericWithLifetime<T>) {} // implied T: 'a

fn lifetime_bound_implied_fn_1<'b, U>(x: &'b U) {} // implied U: 'b

fn do_not_require_lifetime_bound<'a, T>() {} // bound not implied

trait Trait<'a, T> {} 

impl<'a,T> Trait<'a, T> for &'a T {}  // implied T: 'a

struct A<T:Clone>(T);

fn doest_specify_T_clone<T: Clone>(X: A<T>) {}

fn main() {
    println!("Hello, world!");
}
