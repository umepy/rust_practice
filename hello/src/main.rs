use core::num;

fn main() {
    // Numerical types
    let i1 = 5;
    let i2: i8 = 10;

    // String types
    let s1: String = String::from("Hello");
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    // Tuples
    let mut tup = (1, "2");
    tup.0 = 10;
    tup.1 = "20";

    // Arrays
    let mut arr = [0, 1, 2];
    let arr2 = [0; 10];
    for i in 0..arr2.len() {
        println!("{}", arr2[i]);
    }

    // Structs
    struct Person {
        name: String,
        age: i32,
    }

    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown { x: i32, y: i32 },
    }
    let e1 = Event::KeyDown(10);
    let e2 = Event::MouseDown { x: 10, y: 20 };

    let result: Result<i32, String> = Ok(200);
    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let code = result?;
        println!("code {}", code);
        Ok(100)
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![0; 5];

    // mut and immute
    let immut_val = 10;
    let mut mut_val = 20;
    mut_val += immut_val;

    // type
    let v1: u64 = 10;
    let v2 = 20u64;

    let number = 1;
    if 0 < number {
        println!("{}", number);
    } else if number > 0 {
        println!("{}", number);
    } else {
        println!("{}", number);
    }

    // if are formula
    let number = 1;
    let number = if number == 1 { 10 } else { 20 };

    // example of loop
    let mut count = 0;
    let result = loop {
        println!("{}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };

    let mut count = 0;
    while count < 10 {
        println!("{}", count);
        count += 1;
    }

    let count: i32;
    for count in 0..10 {
        println!("{}", count);
    }

    // rust can name loop and for and while
    'main: loop {
        println!("loop");
        'sub: for i in 0..10 {
            println!("{}", i);
            if i == 5 {
                break 'main;
            }
        }
    }
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}
