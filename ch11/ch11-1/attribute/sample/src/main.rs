use funclog::funclog;

fn main() {
    hello()
}

#[funclog]
fn hello() {
    println!("Hello World");
}