use std::usize;

const RADIX_LENGTH: usize = 20;
const RADIX: usize = 1 << RADIX_LENGTH;
const BIT_MASK: usize = RADIX - 1;

struct CompactHashmap {
    keys: Vec<u64>,
    values: Vec<i8>,
    jump_table: [usize; RADIX + 1]
}

impl CompactHashmap {
    pub fn new(mut data: Vec<(u64, i8)>) -> CompactHashmap {
        data.sort_by_key(|x| x.0);
        let mut keys: Vec<u64> = data.iter().map(|x| x.0).collect::<Vec<_>>();
        let mut values: Vec<i8> = data.iter().map(|x| x.1).collect::<Vec<_>>();

        let mut counter = [0 as usize; RADIX];
        for i in 0..keys.len() {
            let mask = (keys[i] >> (64 - RADIX_LENGTH)) as usize & BIT_MASK;
            
            counter[mask] += 1;
        }

        let mut jump_table = [0 as usize; RADIX + 1];
        for r in 1..(RADIX + 1) {
            jump_table[r] = jump_table[r - 1] + counter[r - 1];
        }

        return CompactHashmap {keys, values, jump_table};
    }   
}