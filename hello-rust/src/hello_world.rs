pub struct HelloWorld {
    counter: u32,
}

impl HelloWorld {
    pub fn new(counter: u32) -> Self {
        HelloWorld { counter }
    }

    // 所有権が移動
    pub fn into_u32(self) -> u32 {
        self.counter
    }

    pub fn say(&self) {
        println!("カウンター値は{}", self.counter);
    }
    pub fn countup(&mut self) {
        self.counter += 1;
    }

    // 第1引数がself か&mut self であれば自身のメソッドを呼び出せる。
    pub fn greet(&mut self){
        self.countup();
        self.say();
    }
}
