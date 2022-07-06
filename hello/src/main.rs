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
}
