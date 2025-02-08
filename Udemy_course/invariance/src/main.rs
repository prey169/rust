/// --------------------------------------
/// Invariance
/// --------------------------------------

// fn accepts_vec(str_vec: &mut Vec<&str>, s: &str) {
//     str_vec.push(s);
// }
// fn main() {
//     let mut vec: Vec<&'static str> = vec!["", ""];
//     let non_static_str = &*String::from("");
//     accepts_vec(&mut vec, non_static_str);
// }

fn update_reference<'longer, 'shorter>(mut u_1: &'longer mut u8, mut u_2: &'shorter mut u8)
where
    'longer: 'shorter,
{
    u_2 = u_1; // &'shorter mut u8 = &'longer mut u8
}
fn main() {
    let mut x1: u8 = 8;
    let mut x2: u8 = 10;
    let mut y = &mut x1;
    {
        let mut z = &mut x2;
        update_reference(y, z);
    }
}
