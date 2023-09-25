pub mod merhash;

#[cfg(test)] // 测试模块
mod tests {
    use crate::merhash::mersenne_hash;

    #[test]
    fn mersenne_hash_works() {
        let seed = String::from("jdxjp");
        let hash = mersenne_hash(&seed);
        assert_eq!(2000375, hash);
    }
}