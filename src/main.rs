fn main() {
    let file = std::fs::read_to_string("digits_two.txt")
        .expect("Could not find digits file, please ensure it is in project root");

    let count = &std::env::args().collect::<Vec<String>>()[1].parse::<usize>().unwrap();

    let mut i: usize = 0;
    for _ in 0..*count {
        println!("[{}] :: {}", i, file[i..i+3].to_string());
        i += 3;
    }
}
