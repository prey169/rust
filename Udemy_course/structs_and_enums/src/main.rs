//Struct life baby

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }

    fn display_car_info(&self) {
        println!(
            "Owner: {}, Year: {}. Price: {}",
            self.owner, self.year, self.price
        )
    }

    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name,
            year: year,
            fuel_level: 0.0,
            price: 0,
        }
    }

    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    fn sell(self) -> Self {
        self
    }
}
fn main() {
    let mut my_car = Car {
        owner: String::from("Prey169"),
        year: 2010,
        fuel_level: 0.0,
        price: 5000,
    };

    let car_year = my_car.year;
    my_car.fuel_level = 30.0;
    //let extracted_owner = my_car.owner;
    // println!("Owner is: {}", my_car.owner) // this would error if the above exists as it is a partitial borrow
    // we would need to do the following instead
    let extracted_owner = my_car.owner.clone();

    let another_car = Car {
        owner: "new_name".to_string(),
        ..my_car /* this indicates a partial move, if we do a full move, we will move any heap
                 allocated variables out of the my_car variable! such as:

                 my_car
                 println!("Owner is: {}", my_car.owner) // this would fail as my_car would no longer own the owner value
                 */
    };

    // Tuple Structs
    let point_2D = (1, 3);
    let point_3D = (4, 10, 13);

    struct Point_2D(i32, i32);
    struct Point_3D(i32, i32, i32);

    let point = Point_2D(1, 3);
    let point = Point_3D(4, 10, 13);

    // Unit Struct
    struct ABC;

    my_car.display_car_info();
    my_car.refuel(10.5);
    let new_owner = my_car.sell();
    // my_car.refuel(10.5); running this again will result in borrowed of moved value since we moved to new_owner

    let new_car = Car::new("XYZ".to_string(), 2020);

    // Enum life baby aka dicts
    let weekday = vec![
        "Monday".to_string(),
        "Tuesday".to_string(),
        //... not ideal
    ];

    let day = weekday[1].clone(); //needs to be a clone because we cant take a day from a vec
    let day = WeekDay::Saturday;
    

    let participant = TravelType::Car(200.0);
    println!("Allowance of participant is: {}", participant.travel_allowance());


}


enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum TravelType{
    Car(f32),
    Train(f32),
    Airplane(f32),
}

impl TravelType{
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Airplane(miles) => miles * 5.0,
        };
        allowance
    }
}
