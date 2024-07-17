mod tetri_core;

use tetri_core::tetri_core::{Mino, Tetri};

fn main() {

    let tester = Tetri::new(0, 3, 21, Mino::T); 

    println!("tester : {}, {}, {}, {:?}" , tester.rot(), tester.x(), tester.y(), tester.id());
}
