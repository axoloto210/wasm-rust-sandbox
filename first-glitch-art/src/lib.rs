#[allow(warnings)]
mod bindings;
use rand::Rng;
use bindings::exports::axoloto210::glitch_art::png_glitchable::{Guest, ScanLine};

struct Component;

impl Guest for Component {
    fn glitch(
        mut scan_line: ScanLine,
    ) -> ScanLine {
        // 乱数発生器
        // thread_rng は非推奨に。rand::rngを使う。
        let mut rng = rand::rng();
        //最初のピクセル値をランダムに設定
        // gen_rangeはrandom_rangeに名前が変わっている。
        scan_line.pixel_data[0] = rng.random_range(0..=255);
        scan_line

    }
}

bindings::export!(Component with_types_in bindings);
