use rand::{thread_rng, Rng};

pub fn xor_sides(left: &Vec<u8>, right: &Vec<u8>) -> Vec<u8> {
    if left.len() != right.len() {
        panic!("lengths not matching, left: {:02x?}, right: {:02x?}", left, right);
    }
    
    left.into_iter()
        .zip(right.into_iter())
        .map(|(left, right)| { 
            left ^ right 
        })
        .collect()
}

pub fn xor_with_key(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let key = data.iter().enumerate().map(|(i, _)| {
        key[(i + 1) % key.len()]
    });
    
    data.into_iter()
        .zip(key.into_iter())
        .map(|(left, right)| { 
            left ^ right 
        })
        .collect()
}

pub fn generate_keys(num_of_keys: u32) -> Vec<Vec<u8>> {
    let length: usize = 128;
    
    let mut keys: Vec<Vec<u8>> = vec![];
    for _ in 0..num_of_keys {
        let key = (0..length).map(|_| {
            thread_rng().gen::<u8>()
        }).collect();
        keys.push(key);
    }
    keys
}