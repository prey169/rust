use std::collections::HashMap;


fn main() {
    let mut person:HashMap<&str, i32> = HashMap::new();

    person.insert("Nouman", 40);
    person.insert("Kamran", 44);
    person.insert("Shah", 55);

    println!("The age is {:?}", person.get("Shah").unwrap());
    if person.contains_key("Shah") {
        println!("He exists!");
    } else {
        println!("He doesn't exist!");
    }
    match person.get("Shah") {
        Some(value) => println!("The value exists {}!", value),
        None => println!("It doesn't exist!"),
    };

    for (name, age) in &person{
        println!("The person {} has an age of {}!", name, age);
    }

    let mut likes:HashMap<&str, &str> = HashMap::new();
    likes.insert("Shah", "apples");
    //This will overwrite apples
    likes.insert("Shah", "mangos");
    println!("The fruit which is liked is {:?}", likes);

    //The following will only update if it isnt set already
    likes.entry("Shah").or_insert("apples");
    println!("The fruit which is liked is {:?}", likes);

    let some_vec = vec![5,5,6,8,7,4,1,6,8,9,0,5,5,5,5];
    let mut freq_vec:HashMap<i32, u32> = HashMap::new();

    // This will first set the value of the key to 0 if it doesnt exist yet.
    // Then freq is set as a mutable pointer to that key's value
    // then freq (while pointing to the value of key: i) is updated by 1
    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("{:?}", freq_vec);

}
