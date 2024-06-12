
use std::io;
use rand::Rng;

pub fn gen_few_bit_u64() {
    let mut rng = rand::thread_rng();
    let num : u64 = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
    //num
}

pub fn magic_gen() {
    
}

pub fn popcnt(num: &u64) {
    
}

// return lookup[(occupied & mask) * magic >> shift];