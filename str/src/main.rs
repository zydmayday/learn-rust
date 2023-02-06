fn main() {
    let a = "AAA".to_string();
    let b = a + " BBB";
    // println!("Hello, world! {}", a); a is assigned to b, so we can not ref a anymore
    println!("Hello, world! {}", b);
}
