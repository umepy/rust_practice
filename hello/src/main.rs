use core::num;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
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

    //match
    let i: i32 = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("misc"),
    }

    enum Color {
        Red,
        Green,
        Blue,
    }
    let c = Color::Red;
    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }

    // range
    for number in 1..10 {
        println! {"{}", number};
    }
    //iterator
    struct Iter {
        current: usize,
        max: usize,
    }
    impl Iterator for Iter {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            self.current += 1;
            if self.current - 1 < self.max {
                Some(self.current - 1)
            } else {
                None
            }
        }
    }

    // macro
    let s = concat!("hello", "world");
    let v = vec![1, 2, 3];
    assert!(100 - 12 == 88);

    let mut handles = Vec::new();
    for x in 1..10 {
        handles.push(thread::spawn(move || {
            println!("heLLLO: {}", x);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    let mut handles = Vec::new();
    let mut shared_data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = shared_data.clone();
        handles.push(thread::spawn(move || {
            let mut shared_data = data_ref.lock().unwrap();
            shared_data[x] += 1;
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) -> &Self {
        println!("{}", self.name);
        //method chain
        self
    }

    fn say_age(&self) -> &Self {
        println!("{}", self.age);
        self
    }
}
