fn main() {
    //Enter a  tempature in Fahrenheit to convert to Celsius
    let temp = 100.0;

    let c_temp = (temp - 32.0) * (5.0 / 9.0);

    println!("That is {c_temp} in Celcius");

    let mut n = 0;
    let mut m = 1;
    let mut z = 1;
    println!("Time to fibonacci!!!");
    loop {
        //println!("{n}");
        println!("{n}");
        z = n + m;
        n = m;
        m = z;

        if n > 4182 {
            break;
        };
    }
    let mut x = 1;
    for number in 1..13 {
        x = number;
        while x > 0 {
            match x {
                1 => println! {"on the first day"},
                2 => println!("second"),
                3 => println!("3 CATS"),
                4 => println!("4 DOGS"),
                5 => println!("5 RINGSSSSS"),
                6 => println!("6 BOXES"),
                7 => println!("7 GGG"),
                8 => println!("8 BURDS"),
                9 => println!("9 SCOOBY"),
                10 => println!("10 BIRDS"),
                11 => println!("11 PIPERS"),
                12 => println!("12 BIRDS"),
                i32::MIN..=0_i32 | 13_i32..=i32::MAX => todo!(),
            };
            x -= 1;
        }
        println!()
    }
}
