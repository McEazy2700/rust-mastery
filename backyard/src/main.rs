pub mod garden;

use crate::garden::vegetables::Asparagus;

fn main() {
    let my_vegy = Asparagus{
        color: String::from("Green"),
        is_healthy: true
    };

    dbg!(&my_vegy);
}
