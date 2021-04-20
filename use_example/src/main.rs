/*

Refer:=> https://rinthel.github.io/rust-lang-book-ko/ch07-03-importing-names-with-use.html

*/

/*
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}

*/

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/*
use TrafficLight::{Red, Yellow};
*/
use TrafficLight::*; //이름공간 내의 모든 아이템을 가져오기 위해서는 * 문법을 이용할 수 있습니다
/*
    *는 글롭(glob) 이라고 부르며, 이는 이름공간 내에 공개된 모든 아이템을 가져올 것입니다. 
    여러분은 글롭을 아껴가며 써야 합니다: 글롭은 편리하지만, 여러분이 예상한 것보다 더 많은 아이템을 끌어와서 
    이름 간의 충돌(naming conflict)의 원인이 될수도 있습니다.
*/

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}

