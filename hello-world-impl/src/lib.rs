#[allow(warnings)]
mod bindings;

use crate::bindings::exports::axoloto210::greet::sayable::Guest;
use crate::bindings::axoloto210::greet::greetable::{name, greet};


struct Component;

impl Guest for Component {
    fn say() -> String{
        let name = name();
        let greetings = greet(&name);
        let mut buffer = Vec::new();

        ferris_says::say(&greetings, 80, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    // greetableの実装が必要
}

bindings::export!(Component with_types_in bindings);
