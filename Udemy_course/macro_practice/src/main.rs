macro_rules! make_struct {
    ($name:ident {$($field:ident: $ty:ty),*}) => {
        struct $name {
            $($field: $ty),*
        }
    };
}

macro_rules! make_functions {

    ($($func_name:ident: $return_type:ty => $return_expr:expr),+) => {
        $(
            fn $func_name() -> $return_type {
                $return_expr
            }
        )+
    };
}

make_struct!(Homie {
    age: i32,
    friend: String
});

make_functions!(foo: i32 => 42, bar: String => "hello world".to_owned());
// fn foo() -> i32 { 42 }
// fn bar() -> String {"hello world".to_owned()}

macro_rules! sum_macro {

    ($($x:expr),*) => {

        {

            let mut sum = 0;

            $(sum += $x;)*

            sum

        }

    };

}

fn main() {
    let friend = Homie {
        age: 24,
        friend: "bill".to_string(),
    };

    let result1 = foo();

    let result2 = bar();

    println!("foo result: {}", result1);

    println!("bar result: {}", result2);

    let result = sum_macro!(1, 2, 3, 4, 5);
    // let result = {
    // let mut sum = 0;
    // sum += 1
    // sum += 2
    // sum += 3
    // sum += 4
    // sum += 5
    // sum
    // };
}
