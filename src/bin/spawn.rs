use std::thread;

fn main() {
    let child = thread::spawn(|| {
        println!("child");
    });
    println!("parent");
    child.join().unwrap();
}
