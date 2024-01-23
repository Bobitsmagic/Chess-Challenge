use std::usize;
use std::collections::HashMap;

const RADIX_LENGTH: usize = 20;
const RADIX: usize = 1 << RADIX_LENGTH;
const BIT_MASK: usize = RADIX - 1;

fn calculate_mask(key: u64) -> usize {
    return (key >> (64 - RADIX_LENGTH)) as usize & BIT_MASK;
}

pub struct CompactHashmap {
    keys: Vec<u64>,
    values: Vec<i8>,
    jump_table: Vec<usize>
}

impl CompactHashmap {
    pub fn empty() -> CompactHashmap {
        return CompactHashmap {
            keys: Vec::new(),
            values: Vec::new(),
            jump_table: vec![0; RADIX + 1]
        };
    }

    pub fn from_hashmap(mut data: HashMap<u64, i8>) -> CompactHashmap {
        return CompactHashmap::new(data.drain().collect::<Vec<_>>());
    }
    pub fn len(&self) -> usize {
        return self.keys.len();
    }

    pub fn store_bytes(&self, buffer: &mut Vec<u8>) {
        for i in 0..self.len() {
            buffer.extend_from_slice(&self.keys[i].to_be_bytes());
            buffer.push(self.values[i] as u8);
        }
    }

    pub fn new(mut data: Vec<(u64, i8)>) -> CompactHashmap {
        
        let mut keys: Vec<u64> = data.iter().map(|x| x.0).collect::<Vec<_>>();
        let mut values: Vec<i8> = data.iter().map(|x| x.1).collect::<Vec<_>>();

        let mut counter = vec![0 as usize; RADIX];
        for i in 0..keys.len() {
            let mask = calculate_mask(keys[i]);
            
            counter[mask] += 1;
        }

        let mut jump_table = vec![0 as usize; RADIX + 1];
        for r in 1..(RADIX + 1) {
            jump_table[r] = jump_table[r - 1] + counter[r - 1];
        }

        return CompactHashmap {keys, values, jump_table};
    }

    pub fn contains_key(&self, key: u64) -> bool {
        let mask = calculate_mask(key);

        let start = self.jump_table[mask];
        let end = self.jump_table[mask + 1];

        for i in start..end {
            if self.keys[i] == key {
                return true;
            }
        }

        return false;
    }

    pub fn get(&self, key: u64) -> Option<i8> {
        let mask = calculate_mask(key);

        let start = self.jump_table[mask];
        let end = self.jump_table[mask + 1];

        for i in start..end {
            if self.keys[i] == key {
                return Some(self.values[i]);
            }
        }

        return None;
    }   
}