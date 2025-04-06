mod hello_world;

fn main() {
    let mut hello_world = hello_world::HelloWorld::new(10,"Hello World".to_string());
    hello_world.say();
    hello_world.countup();
    hello_world.say();

    hello_world.greet();

    // // 所有権が移動。これ以降hello_worldのselfを使用したメソッドは呼び出せない。
    // let counter = hello_world.into_u32();

    // println!("counter {}", counter);

    // println!("{:#?}", hello_world);

    let hello_world_str = hello_world.serialize(); 

    println!("{}", hello_world_str);

    println!("{:?}",hello_world::HelloWorld::deserialize(hello_world_str))

}
