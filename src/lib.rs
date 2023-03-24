use std::cmp::{max, min};

use itertools::Itertools;

use num::complex::Complex;
use num::integer::div_rem;

pub const INVERSES: [[u8; 2]; 4] = [[0, 2], [1, 3], [4, 6], [5, 7]];

/// Checks whether a sequence of moves contains at least
/// one pair of inverse moves that would otherwise
/// cancel out.
pub fn check_for_inverses(moves: &[u8]) -> bool {
    for i in INVERSES {
        if moves.contains(&i[0]) && moves.contains(&i[1]) {
            return true;
        }
    }
    false
}

/// Turns an encoded move into a complex number
fn move_to_complex(move_: u8, a: i32, b: i32) -> Complex<i32> {
    let (n, m): (u8, u8);
    // Split into n and m - n is either 0 or 1, m is 0,1,2,3.
    (n, m) = div_rem(move_, 4);
    let mut c_num: Complex<i32> = Complex::new(a, (-1_i32).pow(n as u32) * b);

    // Multiplication by i^m
    c_num = match m {
        0 => c_num * Complex::new(1, 0),
        1 => c_num * Complex::new(0, 1),
        2 => c_num * Complex::new(-1, 0),
        3 => c_num * Complex::new(0, -1),
        _ => unreachable!(), // m is mod 4, so 4+ is unreachable
    };

    c_num
}

#[derive(Debug)]
pub struct DispAndMoves {
    pub a: i32,
    pub b: i32,
    pub displacement: Complex<i32>,
    pub moves: Vec<u8>,
}

impl DispAndMoves {
    pub fn new(a: i32, b: i32, displacement: Complex<i32>, moves: Vec<u8>) -> DispAndMoves {
        DispAndMoves {
            a,
            b,
            displacement,
            moves,
        }
    }
}

pub fn get_min_displacement(a: i32, b: i32, steps: usize) -> DispAndMoves {
    let mut curr_min = DispAndMoves::new(a, b, Complex::new(1000, 1000), vec![0, 0, 0]);

    // Generates a cartesian product of `steps` number of 0..8 intervals.
    // Equivalent to Python: `itertools.product(range(0,8), repeat=steps)`
    for moves in (0..steps).map(|_| 0..8).multi_cartesian_product() {
        if check_for_inverses(&moves) {
            continue;
        }

        let res: Complex<i32> = moves.iter().fold(Complex::new(0, 0), |sum, move_| {
            sum + move_to_complex(*move_, a, b)
        });

        if res != Complex::new(0, 0) && res.norm_sqr() < curr_min.displacement.norm_sqr() {
            curr_min.displacement = res;
            curr_min.moves = moves;
        }
    }
    curr_min
}
#[derive(Debug, Default)]
pub struct ComplexBox {
    pub north: i32,
    pub west: i32,
    pub south: i32,
    pub east: i32,
}

impl ComplexBox {
    pub fn new(north: i32, west: i32, south: i32, east: i32) -> ComplexBox {
        ComplexBox {
            north,
            west,
            south,
            east,
        }
    }
    pub fn get_area(&self) -> u64 {
        // Due to the way NWSE is calculated, the difference cannot
        // ever be negative. (east > west, north > south)
        let width = (self.east - self.west) as u64;
        let height = (self.north - self.south) as u64;
        width * height
    }
    pub fn update_from_complex(&mut self, c_num: Complex<i32>) {
        self.north = max(self.north, c_num.im);
        self.west = min(self.west, c_num.re);
        self.south = min(self.south, c_num.im);
        self.east = max(self.east, c_num.re);
    }
}

/// TODO
pub fn get_min_rect_area(lam: &DispAndMoves, a: i32, b: i32) -> (Vec<u8>, u64) {
    // The moves that yield the minimal area
    let mut curr_min: Vec<u8> = lam.moves.clone();
    // Current minimal area
    let mut min_area: u64 = u64::MAX;

    for move_perm in curr_min.clone().into_iter().permutations(curr_min.len()) {
        let mut res = Complex::new(0, 0);
        let mut c_box = ComplexBox::default();
        for move_ in move_perm.clone().into_iter() {
            res += move_to_complex(move_, a, b);
            c_box.update_from_complex(res);
        }
        if min_area > c_box.get_area() {
            min_area = c_box.get_area();
            curr_min = move_perm;
        }
    }

    (curr_min, min_area)
}
