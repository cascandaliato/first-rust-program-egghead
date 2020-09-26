use std::io;
use std::process;

fn main() {
    // println!("Hello, world!");

    // let mut name = "Foo";
    // println!("{}", name);
    // name = "Bar";
    // println!("{}", name);

    // let name = "John";
    // let another_name = "Doe";
    // println!("{} {}", name, another_name);

    // let first = "John".to_string();
    // let last = "Doe".to_string();
    // say_name(first, last)

    // let first_name = "John".to_string();
    // say_first_name(&first_name);
    // say_first_name(&first_name);

    // println!("Please enter your name: ");
    // let mut name = String::new();
    // io::stdin().read_line(&mut name);
    // println!("Hello {}", name)

    loop {
        // println!("Please enter a second number: ");
        // let mut first = String::new();
        // io::stdin().read_line(&mut first);
        // // let a: u32 = first.trim().parse().unwrap();
        // // let a: u32 = first.trim().parse().expect("This is not a valid number");
        // // let mut a: u32 = 0;
        // let a: u32;
        // match first.trim().parse() {
        //     Ok(val) => {
        //         a = val;
        //     }
        //     Err(_err) => {
        //         println!("This is not a valid number");
        //         process::exit(1);
        //     }
        // }
        // println!("Please enter a second number: ");
        // let mut second = String::new();
        // io::stdin().read_line(&mut second);
        // // let b: u32 = second.trim().parse().unwrap();
        // // let b: u32 = second.trim().parse().expect("This is not a valid number");
        // // let mut b: u32 = 0;
        // let b: u32;
        // match second.trim().parse() {
        //     Ok(val) => {
        //         b = val;
        //     }
        //     Err(_err) => {
        //         println!("This is not a valid number");
        //         process::exit(1);
        //     }
        // }

        println!("Please enter a first number: ");
        let a = read_user_input();
        println!("Please enter a first number: ");
        let b = read_user_input();
        let result = sum(a, b);
        println!("{} + {} = {}", a, b, result)
    }
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}

// fn say_name(first: String, last: String) {
//     println!("{} {}", first, last);
// }

// fn say_first_name(first: &String) {
//     println!("{}", first);
// }

fn read_user_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let digit: u32;
    match input.trim().parse() {
        Ok(val) => {
            digit = val;
        }
        Err(_err) => {
            println!("This is not a valid number");
            process::exit(1);
        }
    }

    digit
}
