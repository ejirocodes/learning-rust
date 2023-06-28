fn main() {
    let x = 12;
    println!("The value of x is {}", x);
    let x = "Anita";
    println!("The value of x is {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 108_000;
    println!("The value of x is {}", THREE_HOURS_IN_SECONDS);
    let character = 'e';

    // COMPOUND TYPES
    let tuple = ("Getting started with rust", 100_200);
    let (name, count) = tuple;
    let count = tuple.1;
    let name = tuple.0;

    // arrays
    let error_codes = [100, 200, 300, 404];
    let not_found = error_codes[3];

    // create an array of value 2, 8 times [2, 2, 2, 2, 2, 2]
    let byte = [2; 6];
    print!("{:?}", byte);

    invoke_windows_api(200, 300);
    control_flow();
}

// FUNCTIONS
fn invoke_windows_api(x: i32, y: i32) -> i32 {
    println!("Calling the invoke_windows_api api..., {}", x);
    println!("Calling the invoke_windows_api api..., {}", y);

    let sum = x + y;
    // explicit return
    return sum;
    // implicit return
    // sum
    // or
    // x + y
}

fn control_flow() {
    // CONDITIONALS
    let number = 8;

    if number < 10 {
        println!("The first condition is true");
    } else if number < 20 {
        println!("The second condition is true");
    } else {
        println!("The third condition is true");
    }

    let condition = false;
    let mut num_condition = if condition == true { 5 } else { 6 };

    // LOOPS
    let result = loop {
        num_condition = num_condition + 1;
        if num_condition > 200 {
            break num_condition;
        };
    };
    println!("Loop infinitely... {}", result);

    // while loop
    let mut response = 33;
    while response != 0 {
        println!("while loop... {}", response);
        response -= 1;
    }
    // for in loop
    let arr = [1, 2, 3, 4, 5];
    for number in arr.iter() {
        println!("{}", number);
    }

    for num in 0..12 {
        println!("{}", num);
    }

    let s1 = String::from("Hello world!");
    let s2 = s1;

    println!("{}", s2);

    let mut s = String::from("Hello");

    change(&mut s);
}

fn change(strng: &mut String) {
    strng.push_str("World!!!");
    println!("{}", strng);
}
