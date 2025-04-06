use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct HelloWorld {
    counter: u32,
    message: String,
}

impl Into<u32> for HelloWorld {
    fn into(self) -> u32 {
        self.into_u32()
    }
}

impl Into<u64> for HelloWorld {
    fn into(self) -> u64 {
        self.into_u32() as u64 // into_u32の返り値をu64にキャスト
    }
}

impl  HelloWorld {
    pub fn serialize(self)->String{
        format!("{}\t{}", self.counter, self.message)
    }    

    pub fn deserialize(input:String)->Result<HelloWorld, ParseIntError>{
        let find_result = input.find('\t');
        let index = find_result.unwrap_or(0);
        let (first, second) = input.split_at(index);

        let counter = u32::from_str(first)?;

        let message = if index > 0 {
            second.trim_start().to_string()
        }else {
            second.to_string()
        };
        Ok(HelloWorld::new(counter, message))
    }
}

impl HelloWorld {
    pub fn new(counter: u32, message: String) -> Self {
        HelloWorld { counter, message }
    }

    // 所有権が移動
    pub fn into_u32(self) -> u32 {
        self.counter
    }

    pub fn say(&self) {
        println!("{}({}回目)", self.message, self.counter);
    }
    pub fn countup(&mut self) {
        self.counter += 1;
    }

    // 第1引数がself か&mut self であれば自身のメソッドを呼び出せる。
    pub fn greet(&mut self) {
        self.countup();
        self.say();
    }
}
