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
}
