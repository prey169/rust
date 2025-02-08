// --------------------------------
// Lifetime bounds
// --------------------------------

fn largest<'a, 'b>(x: &'a u8, y: &'b u8) -> &'a u8
where
    'b: 'a,
{
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let a = largest(&7, &1);
    println!("{a}");
}
