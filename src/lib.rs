use std::hash::{BuildHasher, Hasher, RandomState};

pub fn get_random_hash_u64() -> u64 {
    RandomState::new().build_hasher().finish()
}

pub fn random() -> f64 {
    let mut random_num: f64 = get_random_hash_u64() as f64;
    while random_num >= 1.0 {
        random_num /= 10.0;
    }
    random_num
}

pub fn random_u64() -> u64 {
    let mut random_num = get_random_hash_u64();
    while random_num >= 1 {
        random_num /= 10;
    }
    random_num
}

pub fn uniform(a: f64, b: f64) -> f64 {
    a + (b - a) * random()
}

pub fn randint(a: u64, b: u64) -> u64 {
    if a == 0 {
        return get_random_hash_u64() % b;
    }
    a + (b - a) * random_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = random();
        assert!(result < 1.0);
    }
}
