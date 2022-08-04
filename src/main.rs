use cchat::start;

fn main() {
    println!("Starting server");
    start("0.0.0.0".to_string(), 24268, 4);
}
