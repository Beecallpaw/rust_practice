fn main() {
    let a = vec![1, 2, 3];
    let sum: usize = a.iter().sum();
    let mean: usize = sum / a.len();
    println!("{}", mean);
    let x = read_username();
    println!("{:?}", x);
}
fn read_username() -> Result<String, std::io::Error> {
    std::fs::read_to_string("hello.txt")
}
