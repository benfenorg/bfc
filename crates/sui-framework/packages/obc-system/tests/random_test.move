#[test_only]
module obc_system::random_test {
    use obc_system::random;

    #[test]
    fun test_rand_n_bench() {
        let random = random::new(0);
        let n = 0;
        while (n < 100) {
            random::rand_n(&mut random, 1000000);
            n = n + 1
        }
    }

    #[test]
    fun test_rand_bench() {
        let random = random::new(0);
        let n = 0;
        while (n < 100) {
            random::rand(&mut random);
            n = n + 1
        }
    }

    #[test]
    fun test_with_seed_0() {
        let random = random::new(0);
        let n = 0;
        while (n < 100000) {
            let r1 = random::rand(&mut random);
            let r2 = random::rand(&mut random);
            let r3 = random::rand(&mut random);
            assert!(r1 != 0 || r2 != 0 || r3 != 0, 0);
            assert!(!((r1 == r2) && (r2 == r3)), 0);
            n = n + 1;
        }
    }
}