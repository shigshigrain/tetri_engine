mod tetri_core;
mod tetri_data;
use tetri_core::tetri_core::{Mino, Tetri, TetriDebug, TetriManager, TetriObj};
use std::io;

fn main() {

    //let tester = Tetri::new(0, 3, 21, Mino::T); 
    //println!("tester : {}, {}, {}, {:?}" , tester.rot(), tester.x(), tester.y(), tester.id());
    let mut my_rng = rand::rng();
    let mut test_obj = TetriObj::new();
    test_obj.init(&mut my_rng, 1);

    //println!("{:?}", test_obj.next());

    loop {
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)  
            .expect("failed to read");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match input {
            0 => { 
                test_obj.hard_drop();
            },
            1 => {
                break;
            },
            2 => {
                break;
            },
            _ => (),
        };

        test_obj.paste_mino();
        test_obj.replenish_current();
        test_obj.output_field_on_current();

    };

    print!("Exit!");


}
