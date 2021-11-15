use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    println!("Hello, world!");
}
