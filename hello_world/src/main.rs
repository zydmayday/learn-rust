fn hello() {
    let a = 'a';
    let b = 'b';
    let c = 'c';
    let nums = [a, b, c];
    for i in nums.iter() {
        println!("{}", i)
    }
}

fn main() {
    hello()
}
