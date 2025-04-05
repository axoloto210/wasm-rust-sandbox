use hello_world::countup;

mod hello_world;

fn main() {
    let mut hello_world = hello_world::new_hello_world(0);
    hello_world::say(&hello_world);
    countup(&mut hello_world);
    hello_world::say(&hello_world);

}
