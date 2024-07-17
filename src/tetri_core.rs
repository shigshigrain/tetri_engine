

pub mod tetri_core {
    use std::collections::VecDeque;
    use rand::{rngs::ThreadRng, seq::SliceRandom};

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
    pub enum KindTS{
        None,
        Mini,
        Single,
        Double,
        Triple,
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
        fn new() -> TetriObj;
        fn init(&mut self, _rng: &mut ThreadRng, _id: u32) -> u32;
    }

    #[derive(Debug,Clone)]
    pub struct TetriObj {
        id: u32,
        current: i8,
        hold: i8,
        btb: i32,
        combo: i32,
        kind_srs: i8,
        kind_ts: KindTS,
        next: VecDeque<i8>,
        next_seed: Vec<i8>,
        field: Vec<Vec<i8>>,
    }
    
    impl TetriObj {
        pub fn next(&self) -> &VecDeque<i8> {
            &self.next
        }
    }

    impl TetriManager for TetriObj{
        fn new() -> TetriObj {
            TetriObj { 
                id : 0,
                current : 0,
                hold : 0,
                btb : 0,
                combo : 0,
                kind_srs : 0,
                kind_ts : KindTS::None,
                next : VecDeque::new(),
                next_seed : vec![1, 2, 3, 4, 5, 6, 7],
                field : vec![vec![0; 10]; 45],
            }
        }

        fn init(&mut self, _rng: &mut ThreadRng, _id: u32) -> u32 {
            self.id = _id;
            self.current = 0;
            self.hold = 0;
            self.btb = 0;
            self.combo = 0;
            self.kind_srs = 0;
            self.kind_ts = KindTS::None;
            //self.next_seed = vec![1, 2, 3, 4, 5, 6, 7];
            self.next_seed.shuffle(_rng);
            self.next.clear();
            for _n in &self.next_seed {
                self.next.push_back(_n.clone());
            }
            for col in self.field.iter_mut(){
                for row in col.iter_mut(){
                    *row = 0;
                }
            }
            self.field = vec![vec![0; 10]; 45];

            return self.id;
        }

        


    }



}