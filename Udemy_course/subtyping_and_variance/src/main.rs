// --------------------------------
// Subtyping and variance
// --------------------------------

//Subtype
// X: Y
// 'x: 'y => 'x outlives 'y

//Variance
// 'x: 'y => T<'x> relationship with T<'y> ?

// Types of variance
//      1. Covariance: if x: 'y then T<'x>: T<'y>
//                     if 'static: 'a then 'static T: 'a T, assume T = %str
//                     &'static str: &'a str
//      2. Contravariance
//      3.

fn accepts_str(s: &str) {}
// fn accepts_str(s: &'a str)

// We can provide fn(&'a str) but also fn(&'static str)

fn main() {
    let s_1 = String::from("");
    let s_2 = ""; //'static
                  /*
                  let s_4: &str;
                  {
                      let s_2 = "";
                      s_4 = s_2;
                  }
                  println!("{s_4}")
                  */
    let s_3 = &*s_1; // 'a

    accepts_str(s_2); //'static str
    accepts_str(s_3); //'a str

    
    let mut x = &*s_1; // 'a
    let mut y &'static str = ""; // 'static

    //x = y; // y: &'a str however 'static:'a => &'static: &'a str
    //y = x; //not valid
}
