mod shapes {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle {radius}
        }

        pub fn new_1(radius: f32) -> Result<Circle, String> {
            if radius >= 0.0 {
                Ok(Circle {radius})
            } else {
                Err(String::from("radius should be positive"))
            }
        }
        
        // should not use panic in real world. ideally handle errors gracefully
        pub fn new_2(radius: f32) -> Circle {
            match radius {
                -10.0..=0.0 => panic!("is between -10.0 and 0.0"),
                ..=-10.0 => panic!("is lesser then -10.0"),
                _ => Circle {radius}, 
            }
        }
    
        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

fn some_private_fn() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_circle_should_contain_smaller() {
        let larger_circle = shapes::Circle::new(5.0);
        let smaller_circle = shapes::Circle::new(2.0);
        assert_eq!(larger_circle.contains(&smaller_circle), 
        true,
        "Custom Failure message");

        assert_ne!(larger_circle.contains(&smaller_circle), false);
        assert!(larger_circle.contains(&smaller_circle)); // just checks for true
    }

    #[test]
    fn smaller_circle_should_not_contain_larger() {
        let larger_circle = shapes::Circle::new(5.0);
        let smaller_circle = shapes::Circle::new(2.0);
        assert_eq!(smaller_circle.contains(&larger_circle), false);
    }

    #[test]
    fn should_not_create_circle() -> Result<(), String> {
        let some_circle = shapes::Circle::new_1(-1.0)?; //expects to fail
        Ok(())
    }

    #[test]
    fn can_test_private_functions(){
        some_private_fn();
    }

    #[test]
    //#[should_panic]
    #[should_panic(expected = "is lesser then -10.0")] // panic's and expects a matching string
    fn should_not_create_and_panic() {
        let some_circle = shapes::Circle::new_2(-11.0);
    }

    #[test]
    #[ignore]
    fn huge_test() {
        //code that runs for hours
    }
}
