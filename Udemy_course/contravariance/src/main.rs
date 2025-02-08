/// --------------------------------------
/// Contravariance
/// --------------------------------------

fn str_fn(f: fn(&str)) {}
fn accepts_str_1(f: fn(s: &'static str)) {}
fn accepts_str_2(f: &str) {}

fn main() {
    str_fn(accepts_str_1);
    str_fn(accepts_str_2);

    let mut x: fn(&str) = accepts_str_2
    let mut y: fn(&'static str) = accepts_str_1
}
