use crate::math::Vec3;
use crate::math::Vec2;

use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg32;

pub struct Permutation {
    pub x: [u8; 256],
    pub y: [u8; 256],
    pub z: [u8; 256],
}

impl Permutation {
    pub fn new(seed: u64) -> Self {
        let mut xrng = Pcg32::seed_from_u64(seed.wrapping_add(48576675488));
        let mut yrng = Pcg32::seed_from_u64(seed.wrapping_add(37568376973));
        let mut zrng = Pcg32::seed_from_u64(seed.wrapping_add(97857358375));

        let mut x = PERMUTATION;
        let mut y = PERMUTATION;
        let mut z = PERMUTATION;

        x.shuffle(&mut xrng);
        y.shuffle(&mut yrng);
        z.shuffle(&mut zrng);
        
        Self {
            x, y, z
        }
    }
    
    pub fn index3(&self, n: Vec3<i32>) -> u8 {
        self.x[(n.x() & 255) as usize] ^
        self.y[(n.y() & 255) as usize] ^
        self.z[(n.z() & 255) as usize]
    }

    pub fn index2(&self, n: Vec2<i32>) -> u8 {
        self.x[(n.x() & 255) as usize] ^
        self.z[(n.z() & 255) as usize]
    }

    pub fn index(&self, n: i32) -> u8 {
        self.x[(n & 255) as usize]
    }
}

pub const PERMUTATION: [u8; 256] = [
    151,160,137,91,90,15,
    131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
    190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
    88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
    77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
    102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
    135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
    5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
    223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
    129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
    251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
    49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
    138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180,
];

pub fn szudzik_hash2(n: Vec2<i32>) -> i32 {
    if n.0 >= n.1 {
        n.0 * n.0 + n.0 + n.1
    } else {
        n.1 * n.1 + n.0
    }
}

pub fn prime_mul_hash2(n: Vec2<i32>) -> i32 {
    n.0 * 73856093 + n.1 * 19349663
}

pub fn fnv_hash2(n: Vec2<i32>) -> u64 {
    let mut hash = 2166136261u64;
    hash ^= n.0 as u64;
    hash = hash.wrapping_mul(16777619);
    hash ^= n.1 as u64;
    hash = hash.wrapping_mul(16777619);
    hash
}

fn morton_hash2(x: u32, y: u32) -> u64 {
    fn part1by1(n: u32) -> u64 {
        let mut n = n as u64 & 0x00000000ffffffff;
        n = (n | (n << 16)) & 0x0000ffff0000ffff;
        n = (n | (n << 8)) & 0x00ff00ff00ff00ff;
        n = (n | (n << 4)) & 0x0f0f0f0f0f0f0f0f;
        n = (n | (n << 2)) & 0x3333333333333333;
        n = (n | (n << 1)) & 0x5555555555555555;
        n
    }

    (part1by1(x) | (part1by1(y) << 1)) as u64
}
