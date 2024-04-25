// Problem 1:
/* In this exercise, you will be working on creating a student management system
using Rust. The system should allow you to store and retrieve student information
based on their unique ID. For ease of work, the student structure is already
created in the code below

Next, create a StudentManager structure containing a field of student, which
will essentially be a hashmap where the key part will be an integer representing
unique ID of student and the value part will be the complete details of the
students contained in the student structure.

The StudentManager should implement the following methods:
1. new() -> Self: A constructor that initializes an empty student manager.

2. add_student(&mut self, student: Student) -> Result<(), String>:
Adds a student to the manager.
If the student's ID already exists, return an error message.
Otherwise, add the student to the manager and return Ok.

3. get_student(&self, id: i32) -> Option<&Student>: Retrieves a student
from the manager based on their ID.
If the student is found, return Some(student). Otherwise, return None.

Your task is to implement the StudentManager structure, and the mentioned methods.
Additionally, provide a sample usage of the student management system by adding
a few students and retrieving their information using the get_student() method.
*/
use std::collections::HashMap;
struct Student {
    id: i32,
    name: String,
    grade: String,
}



struct StudentManager {
    field_of_students: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            field_of_students: HashMap::new(),
        }
    }
    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.field_of_students.contains_key(&student.id) {
            return Err("Already exists!".to_string())
        } 
        let _ = &self.field_of_students.insert(student.id, student);
        Ok(())
    }
    fn get_student(&self, id: i32) -> Option<&Student> {
        match self.field_of_students.get(&id) {
            None => None,
            Some(student) => Some(student),
        }
    }
}

fn main() {
    let mut manager = StudentManager::new();

    let student1 = Student {
        id: 1,
        name: String::from("Alice"),
        grade: String::from("A"),
    };
    let student2 = Student {
        id: 2,
        name: String::from("Bob"),
        grade: String::from("B"),
    };

    manager.add_student(student1).unwrap();
    manager.add_student(student2).unwrap();

    // Retrieve and print student information
    if let Some(student) = manager.get_student(1) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }
    if let Some(student) = manager.get_student(2) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }
    if let Some(student) = manager.get_student(3) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    } 

}
