module bfc_system::random {

    public struct Random has drop, store, copy {
        seed: u64
    }

    public fun new(seed: u64): Random {
        Random {
            seed
        }
    }

    public fun seed(r: &mut Random, seed: u64) {
        r.seed = ((((r.seed as u128) + (seed as u128) & 0x0000000000000000ffffffffffffffff)) as u64)
    }

    public fun rand_n(r: &mut Random, n: u64): u64 {
        r.seed = ((((9223372036854775783u128 * ((r.seed as u128) + 999983)) >> 1) & 0x0000000000000000ffffffffffffffff) as u64);
        r.seed % n
    }


    public fun rand(r: &mut Random): u64 {
        r.seed = ((((9223372036854775783u128 * ((r.seed as u128)) + 999983) >> 1) & 0x0000000000000000ffffffffffffffff) as u64);
        r.seed
    }
    // spec rand {
    //     pragma opaque;
    // }



    public fun seed_rand(r: &mut Random, seed: u64): u64 {


        r.seed = ((((r.seed as u128) + (seed as u128) & 0x0000000000000000ffffffffffffffff)) as u64);
        r.seed = (((9223372036854775783u128 * ((r.seed as u128) + 999983) >> 1) & 0x0000000000000000ffffffffffffffff) as u64);
        r.seed
    }

    #[test]
    fun test_rand_n_bench() {
        let mut random = new(0);
        let mut n = 0;
        while (n < 100) {
            rand_n(&mut random, 1000000);
            n = n + 1
        }
    }

    #[test]
    fun test_rand_bench() {
        let mut random = new(0);
        let mut n = 0;
        while (n < 100) {
            rand(&mut random);
            n = n + 1
        }
    }

    #[test]
    fun test_with_seed_0() {
        let mut random = new(0);
        let mut n = 0;
        while (n < 10) {
            let r1 = rand(&mut random);
            let r2 = rand(&mut random);
            let r3 = rand(&mut random);
            assert!(r1 != 0 || r2 != 0 || r3 != 0, 0);
            assert!(!((r1 == r2) && (r2 == r3)), 0);
            n = n + 1;
        }
    }

    //spec module { pragma verify = false; }
}
