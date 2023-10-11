use std::time::Instant;

fn main() {
    for _ in 0..10 {
        let start = Instant::now();
        let res = count_indices(); 
        let duration = start.elapsed();
        println!("Result: {}", res);
        println!("Time: {:?}", duration);
    }

}

pub fn count_ones() -> u64 {
    let mut sum = 0_u64;
    for i in 0..(1_u64 << 30) {
        sum += i.count_ones() as u64;
    }

    return sum;
}

pub fn count_indices() -> u64 {
    let mut sum = 0_u64;
    for i in 0..(1_u64 << 25) {
        let mut val = 1_u64 << ((i * 1337) & 63);
        val |= 1_u64 << ((i * 34653464) & 63);
        val |= 1_u64 << ((i * 6634533) & 63);

        while val != 0 {
            let index = val.trailing_zeros();
            val ^= 1_u64 << index;
            sum += index as u64;
        }

    }

    return sum;
}

fn filter_iter(value: u64) -> impl Iterator<Item=u8> {
    return (0..64).filter(move |x| value >> x & 1 != 0); 
}

fn jump_iter(mut value: u64) -> impl Iterator<Item=u8> {
    return std::iter::from_fn(move || {
        if value != 0 {
            let index = value.trailing_zeros();
            value ^= 1_u64 << index;
            
            Some(index as u8)
        }
        else {
            None
        }
    });    
}

