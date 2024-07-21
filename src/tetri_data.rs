
pub mod tetri_data{

    pub const BLC_I: [[i8; 16]; 4] = [
        [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
    ];

    pub const BLC_J: [[i8; 16]; 4] = [
        [2, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 2 ,0, 0, 0, 0, 0],
        [0, 2, 2, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 0, 2, 0, 0, 2, 2, 0 ,0, 0, 0, 0, 0],
    ];

    pub const BLC_L: [[i8; 16]; 4] = [
        [0, 0, 3, 0, 3, 3, 3 ,0, 0, 0, 0 ,0, 0, 0, 0, 0],
        [0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 3, 3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0],
        [3, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0],
    ];

    pub const BLC_O: [[i8; 16]; 4] = [
        [0, 4, 4, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 4, 4, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 4, 4, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 4, 4, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    pub const BLC_S: [[i8; 16]; 4] = [
        [0, 5, 5, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 5, 0, 0, 0, 5, 5, 0, 0, 0, 5, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 5, 5, 0, 5, 5, 0, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 5, 5, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0],
    ];

    pub const BLC_T: [[i8; 16]; 4] = [
        [0, 6, 0, 0, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 6, 0, 0, 0, 6, 6, 0, 0, 6, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 6, 6, 6, 0, 0, 6, 0, 0, 0, 0, 0, 0],
        [0, 6, 0, 0, 6, 6, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0],
    ];

    pub const BLC_Z: [[i8; 16]; 4] = [
        [7, 7, 0, 0, 0, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 7, 0, 0, 7, 7, 0, 0, 7, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 7, 0, 0, 0, 7, 7, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 7, 7, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0],
    ];

    pub const BLC_I_LIST: [[(i8, i8); 4]; 4] = [
        [(1, 0), (1, 1), (1, 2), (1, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 1), (1, 1), (2, 1), (3, 1)],
    ];

    pub const BLC_J_LIST: [[(i8, i8); 4]; 4] = [
        [(0, 0), (1, 0), (1, 1), (1, 2)],
        [(0, 2), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (1, 1), (2, 1), (2, 0)],
    ];
    
    pub const BLC_L_LIST: [[(i8, i8); 4]; 4] = [
        [(1, 0), (1, 1), (1, 2), (0, 2)],
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(2, 0), (1, 0), (1, 1), (1, 2)],
        [(0, 0), (0, 1), (1, 1), (2, 1)],
    ];

    pub const BLC_O_LIST: [[(i8, i8); 4]; 4] = [
        [(0, 1), (0, 2), (1, 1), (1, 2)],
        [(0, 1), (0, 2), (1, 1), (1, 2)],
        [(0, 1), (0, 2), (1, 1), (1, 2)],
        [(0, 1), (0, 2), (1, 1), (1, 2)],
    ];

    pub const BLC_S_LIST: [[(i8, i8); 4]; 4] = [
        [(1, 0), (1, 1), (0, 1), (0, 2)],
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(2, 0), (2, 1), (1, 1), (1, 2)],
        [(0, 0), (1, 0), (1, 1), (2, 1)],
    ];

    pub const BLC_T_LIST: [[(i8, i8); 4]; 4] = [
        [(1, 0), (1, 1), (1, 2), (0, 1)],
        [(0, 1), (1, 1), (2, 1), (1, 2)],
        [(1, 0), (1, 1), (1, 2), (2, 1)],
        [(0, 1), (1, 1), (2, 1), (1, 0)],
    ];

    pub const BLC_Z_LIST: [[(i8, i8); 4]; 4] = [
        [(0, 0), (0, 1), (1, 1), (1, 2)],
        [(0, 2), (1, 2), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
        [(0, 1), (1, 1), (1, 0), (2, 0)],
    ];

}

