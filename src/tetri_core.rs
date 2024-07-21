

pub mod tetri_core {
    use std::collections::VecDeque;
    use rand::{rngs::ThreadRng, seq::SliceRandom};
    use crate::tetri_data::tetri_data;

    #[derive(Debug,Clone, Copy,PartialEq, PartialOrd)]
    pub enum Mino{
        None,
        I,
        J,
        L,
        O,
        S,
        T,
        Z,
    }

    #[derive(Debug,Clone, Copy)]
    pub enum KindTS{
        None,
        Mini,
        Single,
        Double,
        Triple,
    }

    #[derive(Debug,Clone, Copy)]
    pub enum OperateCmd{
        None,
        Hold,
        SoftDrop,
        HardDrop,
        RotateCCW,
        RotateFCW,
        MoveLeft,
        MoveRight,
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

        pub fn get_param(&self) -> (i8, i8, i8){
            (self.rot, self.y, self.x)
        }
        
    }

    pub trait TetriManager {
        fn new() -> TetriObj;
        fn init(&mut self, _rng: &mut ThreadRng, _id: u32) -> u32;

        fn replenish_current(&mut self);
        fn operate(&mut self, _cmd: &OperateCmd);
        fn hard_drop(&mut self);
        fn check_index(&self, _cy: &i8, _cx: &i8) -> bool;
        fn check_move(&mut self, _sx: i8, _sy: i8) -> bool;
        fn paste_mino(&mut self);
        fn output_field(& self);
    }

    #[derive(Debug,Clone)]
    pub struct TetriObj {
        id: u32,
        current: Mino,
        hold: Mino,
        btb: i32,
        combo: i32,
        kind_srs: i8,
        kind_ts: KindTS,
        next: VecDeque<Mino>,
        next_seed: Vec<Mino>,
        current_mino: Tetri,
        field: Vec<Vec<i8>>,
    }
    
    impl TetriObj {
        pub fn next(&self) -> &VecDeque<Mino> {
            &self.next
        }

    }

    impl TetriManager for TetriObj{
        fn new() -> TetriObj {
            TetriObj { 
                id : 0,
                current : Mino::None,
                hold : Mino::None,
                btb : 0,
                combo : 0,
                kind_srs : 0,
                kind_ts : KindTS::None,
                next : VecDeque::new(),
                next_seed : vec![Mino::I, Mino::J, Mino::L, Mino::O, Mino::S, Mino::T, Mino::Z],
                current_mino : Tetri::new(0, 3, 21, Mino::T),
                field : vec![vec![0; 10]; 45],
            }
        }

        fn init(&mut self, _rng: &mut ThreadRng, _id: u32) -> u32 {
            self.id = _id;
            self.current = Mino::None;
            self.hold = Mino::None;
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
            self.replenish_current();

            for col in self.field.iter_mut(){
                for blc in col.iter_mut(){
                    *blc = 0;
                }
            }
            self.field = vec![vec![0; 10]; 45];

            return self.id;
        }

        fn replenish_current(&mut self){
            let _nc = self.next.pop_front();
            match _nc {
                Some(_c) => {
                    self.current = _c.clone();
                    self.current_mino.set_mino(_c);
                },
                None => (),
            } 
        }

        fn operate(&mut self, _cmd: &OperateCmd){
            
            match _cmd {
                OperateCmd::None => todo!(),
                OperateCmd::Hold => todo!(),
                OperateCmd::SoftDrop => todo!(),
                OperateCmd::HardDrop => self.hard_drop(),
                OperateCmd::RotateCCW => todo!(),
                OperateCmd::RotateFCW => todo!(),
                OperateCmd::MoveLeft => todo!(),
                OperateCmd::MoveRight => todo!(),
            }

        }

        fn hard_drop(&mut self){
            while self.check_move(0, -1) {
                self.current_mino.add_y(&-1);
            }
        }

        fn check_index(&self, _cy: &i8, _cx: &i8) -> bool {
            let mut tf = false;
            if 0 < *_cy && *_cy <= 45 && 0 <= *_cx && *_cx < 10{
                tf = true;
            }

            tf
        }
        
        fn check_move(&mut self, _sx: i8, _sy: i8) -> bool{
            let (_r, mut _ty, mut _tx) = self.current_mino.get_param();
            _ty += _sy;
            _tx += _sx;
           
            let mut _cnt = 0;
            let mut _myx: (i8, i8) = (0, 0);

            for i in 0..4usize{
                match self.current {
                    Mino::None => {
                        _myx = (0, 0);
                    },
                    Mino::I => {
                        _myx = tetri_data::BLC_I_LIST[_r as usize][i];
                    },
                    Mino::J => {
                        _myx = tetri_data::BLC_J_LIST[_r as usize][i];
                    },
                    Mino::L => {
                        _myx = tetri_data::BLC_L_LIST[_r as usize][i];
                    },
                    Mino::O => {
                        _myx = tetri_data::BLC_O_LIST[_r as usize][i];
                    },
                    Mino::S => {
                        _myx = tetri_data::BLC_S_LIST[_r as usize][i];
                    },
                    Mino::T => {
                        _myx = tetri_data::BLC_T_LIST[_r as usize][i];
                    },
                    Mino::Z => {
                        _myx = tetri_data::BLC_Z_LIST[_r as usize][i];
                    },
                }

                // 配列と見た目のy軸向きを合わせるため、_yはマイナスにする;
                let _ty = _ty - _myx.0;
                let _tx = _tx + _myx.1;
                if self.check_index(&_ty, &_tx){
                    if self.field[_ty as usize][_tx as usize] == 0i8{
                        _cnt += 1;
                    }
                }
                else {
                    continue;
                }

            }

            if _cnt < 4{
                false
            }
            else{
                true
            }
        
        }

        fn paste_mino(&mut self) {
            
            let (_r, mut _ty, mut _tx) = self.current_mino.get_param();

            let mut _cnt = 0;
            let mut _myx: (i8, i8) = (0, 0);

            for i in 0..4usize{
                match self.current {
                    Mino::None => {
                        _myx = (0, 0);
                    },
                    Mino::I => {
                        _myx = tetri_data::BLC_I_LIST[_r as usize][i];
                    },
                    Mino::J => {
                        _myx = tetri_data::BLC_J_LIST[_r as usize][i];
                    },
                    Mino::L => {
                        _myx = tetri_data::BLC_L_LIST[_r as usize][i];
                    },
                    Mino::O => {
                        _myx = tetri_data::BLC_O_LIST[_r as usize][i];
                    },
                    Mino::S => {
                        _myx = tetri_data::BLC_S_LIST[_r as usize][i];
                    },
                    Mino::T => {
                        _myx = tetri_data::BLC_T_LIST[_r as usize][i];
                    },
                    Mino::Z => {
                        _myx = tetri_data::BLC_Z_LIST[_r as usize][i];
                    },
                }

                let _ty = _ty - _myx.0;
                let _tx = _tx + _myx.1;
                if self.check_index(&_ty, &_tx){
                    self.field[(_ty - 1) as usize][_tx as usize] = self.current as i8;
                }
                else {
                    continue;
                }

            }


        }

        fn output_field(&self){
            println!("field -> \n");
            for col in self.field.iter().rev(){
                let mut _cs = String::new();
                for blc in col.iter(){
                    if *blc == 0 {
                        _cs.push_str("□");
                    }
                    else{
                        _cs.push_str("■");
                    }
                }
                println!("{}", _cs);
            }
        }

    }



}