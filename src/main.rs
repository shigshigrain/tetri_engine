mod tetri_core;
mod tetri_data;
use tetri_core::tetri_core::{Mino, Tetri, TetriManager, TetriObj};

fn main() {

    //let tester = Tetri::new(0, 3, 21, Mino::T); 
    //println!("tester : {}, {}, {}, {:?}" , tester.rot(), tester.x(), tester.y(), tester.id());
    let mut my_rng = rand::thread_rng();
    let mut test_obj = TetriObj::new();
    test_obj.init(&mut my_rng, 1);

    println!("{:?}", test_obj.next());
    

}
