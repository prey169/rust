use std::{array, collections::HashMap, io};

fn piggy(mut st: String) ->String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let returned_string: String = if vowels.iter().any(|&v| st.starts_with(v)) {
        let mut a = st.clone();
        a.push_str("-hay");
        a
    } else {
        let mut a = st.chars().next().unwrap();
        st.remove(0);
        st.push_str("-");
        st.push_str(&a.to_string());
        st.push_str("ay");
        st
    };
    returned_string
    
}

fn main() {
    let mut hashcount: HashMap<i32, i32> = HashMap::new();
    let list_o_nums = [1,2,4,6,8,9,0,5,3,2, 22, 22,22];
    //let list_o_nums = list_o_nums.sort();
    let mut vec_o_nums = list_o_nums.to_vec();
    vec_o_nums.sort();
    let mid = vec_o_nums.len() / 2;
    println!("Median is {}", vec_o_nums[mid]);


    for i in vec_o_nums {
        let count = hashcount.entry(i).or_insert(0);
        *count += 1;
    }
    
    println!("mode is {:?}", hashcount.keys().max().unwrap());

    let s1 = piggy("first".to_string());
    let s2 = piggy("apple".to_string());

    println!("{:?}", s1);
    println!("{:?}", s2);

    let mut departments: HashMap<String, Vec<String>>  = HashMap::new();
 
    loop  {
        // take in value to add to array within the hask - Sales: [Name1, name2], marketing:[name3, name4]
        println!("Please lmk your request.");

        let mut request = String::new();

        io::stdin()
            .read_line(&mut request)
            .expect("Failed to read line");

        let (name, dep) = match request.trim().parse::<String>() {
            Ok(_String) => {
                if request.to_lowercase().starts_with("exit"){
                    break;
                }
                let request:Vec<_> = request.trim().split(" ").collect();
                if request[0].to_lowercase() == "add" && 
                request[2].to_lowercase() == "to" &&
                request.len() > 3 {
                (request[1].to_string(), request[3..].join(" ").to_string())
                } else if request[0].to_lowercase() == "show" &&
                request[1].to_lowercase() == "all" {
                    let mut names:Vec<String> = Vec::new();
                    for d in departments.keys() {
                        for name in departments.get(d).unwrap().clone() {
                            names.push(name);
                        }
                    }
                    names.sort();
                    println!("All the names in the company are {:?}", names);
                    continue;
                } else if request[0].to_lowercase() == "show" && request.len() >= 2 {
                    let dep = request[1..].join(" ");
                    let dep = dep.to_string();
                    if departments.contains_key(&dep){
                        let mut names = departments.get(&dep).unwrap().clone();
                        names.sort();
                        println!("The people in {}, are {:?}.", dep, names);
                    } else {
                        println!("Department {} doesn't exist", dep);
                    }
                    continue;
                } else {
                    println!("{:?}: not a valid request", request);
                    println!("Please write 'add name to department'");
                    continue;
                }
            }
            Err(_) => {
                println!("Not a add request!");
                continue;
            }
        };
        if departments.contains_key(&dep){
            let mut arr = departments.get(&dep).unwrap().clone();
            arr.push(name);
            departments.insert(dep, arr.to_vec());
        } else {
            let arr = vec![name];
            departments.insert(dep, arr);
        };
    }
    println!("Departments = {:?}", departments)
}
