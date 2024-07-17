

pub mod tetri_core {
    use std::collections::VecDeque;


    #[derive(Debug,Clone,PartialEq, PartialOrd)]
    pub enum Mino{
        I,
        J,
        L,
        O,
        S,
        T,
        Z,
    }

    #[derive(Debug,Clone)]
    pub struct Tetri {
        rot: i8,
        x: i8,
        y: i8,
        id: Mino,
    }

    impl Tetri{
        pub fn new(rot: i8, x: i8, y: i8, id: Mino) -> Tetri{
            Tetri{
                rot: rot,
                x: x,
                y: y,
                id: id,
            }
        }

        pub fn set_default(&mut self){
            self.rot = 0;
            self.x = 3;
            self.y = 21;
            self.id = Mino::T;
        }

        pub fn set_mino(&mut self, id: Mino){
            self.rot = 0;
            self.x = 3;
            self.y = 21;
            self.id = id.clone();
        }

        pub fn set(&mut self, rot: i8, x: i8, y: i8){
            self.rot = rot;
            self.x = x;
            self.y = y;
        }

        pub fn add_y(&mut self, add : &i8){
            self.y += add;
        }

        pub fn add_x(&mut self, add : &i8){
            self.x += add;
        }

        pub fn compare(&self, other : &Tetri) -> std::cmp::Ordering{

            if self.id < other.id{
                return std::cmp::Ordering::Less
            }
            else if  self.id > other.id{
                return std::cmp::Ordering::Greater
            }

            if self.x < other.x{
                return std::cmp::Ordering::Less
            }
            else if  self.x > other.x{
                return std::cmp::Ordering::Greater
            }

            if self.y < other.y{
                return std::cmp::Ordering::Less
            }
            else if  self.y > other.y{
                return std::cmp::Ordering::Greater
            }

            if self.id == Mino::O{
                // O-minoは回転操作させたくないため;
                return std::cmp::Ordering::Greater
            }
            else{

                if self.rot == other.rot{
                    return std::cmp::Ordering::Equal
                }
                else if self.rot > other.rot{
                    return std::cmp::Ordering::Less
                }
                else{
                    return std::cmp::Ordering::Greater
                }

            }

        }
        
        
        pub fn rot_mut(&mut self) -> &mut i8 {
            &mut self.rot
        }
        
        pub fn set_rot(&mut self, rot: i8) {
            self.rot = rot;
        }
        
        pub fn rot(&self) -> i8 {
            self.rot
        }

        pub fn x(&self) -> i8 {
            self.x
        }
        
        pub fn y(&self) -> i8 {
            self.y
        }
            
        pub fn id(&self) -> &Mino {
            &self.id
        }
        
    }

    pub trait TetriManager {
        fn init(_obj : &mut TetriObj) -> u32{
            _obj.id = 0;
            _obj.next;


            return _obj.id;
        }
    }

    #[derive(Debug,Clone)]
    pub struct TetriObj {
        id: u32,
        current: i8,
        hold: i8,
        next: VecDeque<i8>,
        field: Vec<Vec<i8>>,
    }


}