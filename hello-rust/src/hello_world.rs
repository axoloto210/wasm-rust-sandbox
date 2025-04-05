
pub struct HelloWorld {
    counter: u32,
}

pub fn say(hello_world: &HelloWorld){
    println!("カウンター値は{}",hello_world.counter);
}

pub fn new_hello_world (counter: u32) -> HelloWorld {
    HelloWorld { counter }
}