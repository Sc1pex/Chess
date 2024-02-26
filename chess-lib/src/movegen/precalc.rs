use super::{magic::*, *};
use crate::bitboardindex::BitBoardIdx;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PRECALC: Precalc = Precalc::default();
}

#[derive(Debug, Clone)]
pub struct Precalc {
    pub bishop_magic: Box<[[BitBoard; 512]; 64]>,
    pub rook_magic: Box<[[BitBoard; 4096]; 64]>,
    pub bishop: Box<[(BitBoard, u32); 64]>,
    pub rook: Box<[(BitBoard, u32); 64]>,
    pub pawns: Box<[[BitBoard; 2]; 64]>,
    pub knight: Box<[BitBoard; 64]>,
    pub king: Box<[BitBoard; 64]>,
}

impl Precalc {
    pub fn bishop_attack(&self, square: impl BitBoardIdx, occ: BitBoard) -> BitBoard {
        let occ = occ & self.bishop[square.idx_usize()].0;
        let occ = occ.0.wrapping_mul(BISHOP_MAGIC[square.idx_usize()]);
        let occ = occ >> (64 - self.bishop[square.idx_usize()].1);
        self.bishop_magic[square.idx_usize()][occ as usize]
    }

    pub fn rook_attack(&self, square: impl BitBoardIdx, occ: BitBoard) -> BitBoard {
        let occ = occ & self.rook[square.idx_usize()].0;
        let occ = occ.0.wrapping_mul(ROOK_MAGIC[square.idx_usize()]);
        let occ = occ >> (64 - self.rook[square.idx_usize()].1);
        self.rook_magic[square.idx_usize()][occ as usize]
    }

    pub fn queen_attack(&self, square: impl BitBoardIdx, occ: BitBoard) -> BitBoard {
        self.bishop_attack(square, occ) | self.rook_attack(square, occ)
    }
}

impl Default for Precalc {
    fn default() -> Self {
        Self {
            bishop_magic: calc_bishop_magic(),
            rook_magic: calc_rook_magic(),
            bishop: calc_bishop(),
            rook: calc_rook(),
            pawns: calc_pawns(),
            knight: calc_knight(),
            king: calc_king(),
        }
    }
}

pub fn calc_bishop() -> Box<[(BitBoard, u32); 64]> {
    let mut res = Box::new([(BitBoard(0), 0); 64]);
    for square in 0..64 {
        let a = bishop_attack(square);
        res[square as usize] = (a, a.0.count_ones());
    }
    res
}

pub fn calc_rook() -> Box<[(BitBoard, u32); 64]> {
    let mut res = Box::new([(BitBoard(0), 0); 64]);
    for square in 0..64 {
        let a = rook_attack(square);
        res[square as usize] = (a, a.0.count_ones());
    }
    res
}

pub fn calc_bishop_magic() -> Box<[[BitBoard; 512]; 64]> {
    let mut res = Box::new([[BitBoard(0); 512]; 64]);
    for square in 0..64 {
        let attack_mask = bishop_attack(square);
        let attack_bits = attack_mask.0.count_ones();
        let max_idx = 1 << attack_bits;

        for idx in 0..max_idx {
            let occ = occupancy(idx, attack_mask);
            let magic_idx = occ.0.wrapping_mul(BISHOP_MAGIC[square as usize]) >> (64 - attack_bits);
            res[square as usize][magic_idx as usize] = bishop_attack_blocker(square, occ);
        }
    }
    res
}

pub fn calc_rook_magic() -> Box<[[BitBoard; 4096]; 64]> {
    let mut res = Box::new([[BitBoard(0); 4096]; 64]);
    for square in 0..64 {
        let attack_mask = rook_attack(square);
        let attack_bits = attack_mask.0.count_ones();
        let max_idx = 1 << attack_bits;

        for idx in 0..max_idx {
            let occ = occupancy(idx, attack_mask);
            let magic_idx = occ.0.wrapping_mul(ROOK_MAGIC[square as usize]) >> (64 - attack_bits);
            res[square as usize][magic_idx as usize] = rook_attack_blocker(square, occ);
        }
    }
    res
}

pub fn calc_pawns() -> Box<[[BitBoard; 2]; 64]> {
    let mut res = Box::new([[BitBoard(0); 2]; 64]);
    for square in 0..64 {
        res[square as usize] = [
            pawn_attack(square, Color::White),
            pawn_attack(square, Color::Black),
        ];
    }
    res
}

pub fn calc_knight() -> Box<[BitBoard; 64]> {
    let mut res = Box::new([BitBoard(0); 64]);
    for square in 0..64 {
        res[square as usize] = knight_attack(square);
    }
    res
}

pub fn calc_king() -> Box<[BitBoard; 64]> {
    let mut res = Box::new([BitBoard(0); 64]);
    for square in 0..64 {
        res[square as usize] = king_attack(square);
    }
    res
}

pub fn pawn_attack(square: u64, color: Color) -> BitBoard {
    let mut attacks = BitBoard(0);
    match color {
        Color::White => {
            if (1 << square) & FILE_A == 0 && square <= 56 {
                attacks.set(square + 7);
            }
            if (1 << square) & FILE_H == 0 && square <= 54 {
                attacks.set(square + 9);
            }
        }
        Color::Black => {
            if (1 << square) & FILE_A == 0 && square >= 9 {
                attacks.set(square - 9);
            }
            if (1 << square) & FILE_H == 0 && square >= 7 {
                attacks.set(square - 7);
            }
        }
    };
    attacks
}

pub fn knight_attack(square: u64) -> BitBoard {
    let mut attacks = BitBoard(0);
    let sqb = 1 << square;
    if sqb & FILE_A == 0 && sqb & RANK_78 == 0 {
        attacks.set(square + 15);
    }
    if sqb & FILE_H == 0 && sqb & RANK_78 == 0 {
        attacks.set(square + 17);
    }
    if sqb & FILE_AB == 0 && sqb & RANK_8 == 0 {
        attacks.set(square + 6);
    }
    if sqb & FILE_GH == 0 && sqb & RANK_8 == 0 {
        attacks.set(square + 10);
    }
    if sqb & FILE_H == 0 && sqb & RANK_12 == 0 {
        attacks.set(square - 15);
    }
    if sqb & FILE_A == 0 && sqb & RANK_12 == 0 {
        attacks.set(square - 17);
    }
    if sqb & FILE_GH == 0 && sqb & RANK_1 == 0 {
        attacks.set(square - 6);
    }
    if sqb & FILE_AB == 0 && sqb & RANK_1 == 0 {
        attacks.set(square - 10);
    }

    attacks
}

pub fn king_attack(square: u64) -> BitBoard {
    let mut attacks = BitBoard(0);
    let sqb = 1 << square;
    let file_a = sqb & FILE_A != 0;
    let file_h = sqb & FILE_H != 0;
    let rank_1 = sqb & RANK_1 != 0;
    let rank_8 = sqb & RANK_8 != 0;

    if !file_a {
        attacks.set(square - 1);
        if !rank_1 {
            attacks.set(square - 9);
        }
        if !rank_8 {
            attacks.set(square + 7);
        }
    }
    if !file_h {
        attacks.set(square + 1);
        if !rank_1 {
            attacks.set(square - 7);
        }
        if !rank_8 {
            attacks.set(square + 9);
        }
    }
    if !rank_1 {
        attacks.set(square - 8);
    }
    if !rank_8 {
        attacks.set(square + 8);
    }

    attacks
}

pub fn bishop_attack(square: u64) -> BitBoard {
    let mut attacks = BitBoard(0);

    let (i, j) = (square / 8, square % 8);
    let (mut x, mut y) = (i + 1, j + 1);
    while x < 7 && y < 7 {
        let idx = x * 8 + y;
        attacks.set(idx);
        x += 1;
        y += 1;
    }
    if i > 0 {
        let (mut x, mut y) = (i - 1, j + 1);
        while x > 0 && y < 7 {
            let idx = x * 8 + y;
            attacks.set(idx);
            x -= 1;
            y += 1;
        }
    }
    if j > 0 {
        let (mut x, mut y) = (i + 1, j - 1);
        while x < 7 && y > 0 {
            let idx = x * 8 + y;
            attacks.set(idx);
            x += 1;
            y -= 1;
        }
    }
    if i > 0 && j > 0 {
        let (mut x, mut y) = (i - 1, j - 1);
        while x > 0 && y > 0 {
            let idx = x * 8 + y;
            attacks.set(idx);
            x -= 1;
            y -= 1;
        }
    }

    attacks
}

pub fn rook_attack(square: u64) -> BitBoard {
    let mut attacks = BitBoard(0);

    let (i, j) = (square / 8, square % 8);
    let (x, mut y) = (i, 1);
    while y < 7 {
        let idx = x * 8 + y;
        if y != j {
            attacks.set(idx);
        }
        y += 1;
    }
    let (mut x, y) = (1, j);
    while x < 7 {
        let idx = x * 8 + y;
        if x != i {
            attacks.set(idx);
        }
        x += 1;
    }

    attacks
}

pub fn bishop_attack_blocker(square: u64, blockers: BitBoard) -> BitBoard {
    let mut attacks = BitBoard(0);

    let (i, j) = (square / 8, square % 8);
    let (mut x, mut y) = (i + 1, j + 1);
    while x <= 7 && y <= 7 {
        let idx = x * 8 + y;
        attacks.set(idx);
        if blockers.get(idx) {
            break;
        }
        x += 1;
        y += 1;
    }
    if i > 0 {
        let (mut x, mut y) = (i - 1, j + 1);
        while y <= 7 {
            let idx = x * 8 + y;
            attacks.set(idx);
            if blockers.get(idx) {
                break;
            }

            if x == 0 {
                break;
            }
            x -= 1;
            y += 1;
        }
    }
    if j > 0 {
        let (mut x, mut y) = (i + 1, j - 1);
        while x <= 7 {
            let idx = x * 8 + y;
            attacks.set(idx);
            if blockers.get(idx) {
                break;
            }

            if y == 0 {
                break;
            }
            x += 1;
            y -= 1;
        }
    }
    if i > 0 && j > 0 {
        let (mut x, mut y) = (i - 1, j - 1);
        loop {
            let idx = x * 8 + y;
            attacks.set(idx);
            if blockers.get(idx) {
                break;
            }

            if x == 0 || y == 0 {
                break;
            }
            x -= 1;
            y -= 1;
        }
    }

    attacks
}

pub fn rook_attack_blocker(square: u64, blockers: BitBoard) -> BitBoard {
    let mut attacks = BitBoard(0);

    let (i, j) = (square / 8, square % 8);
    let (mut x, y) = (i + 1, j);
    while x <= 7 {
        let idx = x * 8 + y;
        attacks.set(idx);
        if blockers.get(idx) {
            break;
        }
        x += 1;
    }
    if i > 0 {
        let (mut x, y) = (i - 1, j);
        loop {
            let idx = x * 8 + y;
            attacks.set(idx);
            if blockers.get(idx) {
                break;
            }

            if x == 0 {
                break;
            }
            x -= 1;
        }
    }
    let (x, mut y) = (i, j + 1);
    while y <= 7 {
        let idx = x * 8 + y;
        attacks.set(idx);
        if blockers.get(idx) {
            break;
        }
        y += 1;
    }
    if j > 0 {
        let (x, mut y) = (i, j - 1);
        loop {
            let idx = x * 8 + y;
            attacks.set(idx);
            if blockers.get(idx) {
                break;
            }

            if y == 0 {
                break;
            }
            y -= 1;
        }
    }

    attacks
}
