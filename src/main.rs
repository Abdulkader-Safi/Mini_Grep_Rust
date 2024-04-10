fn main() {
    let args: Vec<String> = std::env::args().collect();

    let query = &args[1];
    let file_name = &args[2];

    println!("{:?}", args);
    println!("Searching for {}", file_name);
    println!("In file {}", query);
}
