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
pub struct LenAndMoves {
    pub displacement: Complex<i32>,
    pub moves: Vec<u8>,
}

impl LenAndMoves {
    pub fn new(displacement: Complex<i32>, moves: Vec<u8>) -> LenAndMoves {
        LenAndMoves {
            displacement,
            moves,
        }
    }
}

pub fn get_min_displacement(a: i32, b: i32, steps: usize) -> LenAndMoves {
    let mut curr_min = LenAndMoves::new(Complex::new(1000, 1000), vec![0, 0, 0]);

    for idx in 0..8_u128.pow(steps as u32) {
        // Convert to octal with front zeros filled
        // and collect into Vec of digits
        // TODO: implement using a less clunky method lol (itertools?)
        let moves: Vec<u8> = format!("{:0fill$o}", idx, fill = steps)
            .chars()
            .map(|d| d.to_digit(10).unwrap() as u8)
            .collect();

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

/// TODO
pub fn get_min_rect_area(lam: &LenAndMoves) -> (Vec<u8>, u64) {
    
    let mut min_area: u64 = u64::MAX;
    let mut curr_min: Vec<u8> = lam.moves.clone();

    for perm in curr_min.clone().into_iter().permutations(curr_min.len()) {

    }

    (vec![0u8], 0)
}
