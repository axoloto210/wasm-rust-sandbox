fn main() {
    let hello_world = new_hello_world(0);
    say(&hello_world);
    say(&hello_world);

}

struct HelloWorld {
    counter: u32,
}

fn say(hello_world: &HelloWorld){
    println!("カウンター値は{}",hello_world.counter);
}

fn new_hello_world (counter: u32) -> HelloWorld {
    HelloWorld { counter }
}