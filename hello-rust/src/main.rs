mod hello_world;

fn main() {
    let hello_world = hello_world::new_hello_world(0);
    hello_world::say(&hello_world);
    hello_world::say(&hello_world);

}
