use super::chunk::{CHUNK_LEN, CHUNK_WIDTH};

/// Each element in this buffer corresponds to an index
/// in a subchunk. Its value is a bitfield indicating
/// what edges of the subchunk it is on, if any.
///
/// Order is -x, +x, -y, +y, -z, +z
pub const CACHED_NEIGHBOUR_CHUNK_BOUNDARIES: [u8; CHUNK_LEN] = {
    let mut result = [0; CHUNK_LEN];
    const W: usize = CHUNK_WIDTH - 1;

    let (mut z, mut x, mut y) = (0, 0, 0);
    while z < CHUNK_WIDTH {
        while x < CHUNK_WIDTH {
            while y < CHUNK_WIDTH {
                let index = y + x * CHUNK_WIDTH + z * CHUNK_WIDTH * CHUNK_WIDTH;

                let mut packed = 0;
                if x == 0 { packed |= 0b000001 }
                if x == W { packed |= 0b000010 }
                if y == 0 { packed |= 0b000100 }
                if y == W { packed |= 0b001000 }
                if z == 0 { packed |= 0b010000 }
                if z == W { packed |= 0b100000 }

                result[index] = packed;
                
                y += 1;
            }
            x += 1;
            y = 0;
        }
        z += 1;
        x = 0;
    }
    
    result  
};
