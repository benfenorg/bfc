#[test_only]
#[allow(unused_function)]
module obc_system::skip_list_test {
    use std::debug;
    use std::vector;

    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    use obc_system::option_u64;
    use obc_system::random;
    use obc_system::skip_list;

    #[test_only]
    fun print_skip_list<V: store>(list: &skip_list::SkipList<V>) {
        debug::print(list);
        if (skip_list::length(list) == 0) {
            return
        };

        let next_score = &skip_list::head(list);
        while (option_u64::is_some(next_score)) {
            let node = skip_list::borrow_node(list, option_u64::borrow(next_score));
            next_score = &skip_list::find_next(list, option_u64::borrow(next_score), false);
            debug::print<skip_list::Node<V>>(node);
        }
    }

    fun check_skip_list<V: store>(list: &skip_list::SkipList<V>) {
        if (skip_list::level(list) == 0) {
            assert!(skip_list::length(list) == 0, 0);
            return
        };

        // Check level 0
        let (
            size,
            opt_next_score,
            _tail,
            prev,
            current_score,
        ) = (
            0,
            &skip_list::head(list),
            option_u64::none(),
            option_u64::none(),
            option_u64::none()
        );
        while (option_u64::is_some(opt_next_score)) {
            let next_score = option_u64::borrow(opt_next_score);
            let next_node = skip_list::borrow_node(list, next_score);
            if (option_u64::is_some(&current_score)) {
                assert!(next_score > option_u64::borrow(&current_score), 0);
            };
            let score = skip_list::get_node_score(next_node);
            let nexts = skip_list::get_node_nexts(next_node);
            assert!(score == next_score, 0);
            if (option_u64::is_none(&prev)) {
                assert!(option_u64::is_none(&prev), 0)
            } else {
                assert!(option_u64::borrow(&prev) == option_u64::borrow(&prev), 0);
            };
            prev = option_u64::some(score);
            _tail = option_u64::some(score);
            //current_score = next_node.score;
            option_u64::swap_or_fill(&mut current_score, score);
            size = size + 1;
            opt_next_score = vector::borrow(&nexts, 0);
        };
        if (option_u64::is_none(&skip_list::tail(list))) {
            assert!(option_u64::is_none(&skip_list::tail(list)), 0);
        } else {
            assert!(option_u64::borrow(&skip_list::tail(list)) == option_u64::borrow(&skip_list::tail(list)), 0);
        };
        assert!(size == skip_list::length(list), 0);

        // Check indexer levels
        let l = skip_list::length(list) - 1;
        while (l > 0) {
            let opt_next_l_score = &skip_list::head_level(list, l);
            let opt_next_0_score = &skip_list::head(list);
            while (option_u64::is_some(opt_next_0_score)) {
                let next_0_score = option_u64::borrow(opt_next_0_score);
                let node = skip_list::borrow_node(list, next_0_score);
                let score = skip_list::get_node_score(node);
                let nexts = skip_list::get_node_nexts(node);
                if (option_u64::is_none(opt_next_l_score) || option_u64::borrow(opt_next_l_score) > score) {
                    assert!(vector::length(&nexts) <= l, 0);
                } else {
                    if (vector::length(&nexts) > l) {
                        assert!(option_u64::borrow(opt_next_l_score) == score, 0);
                        opt_next_l_score = vector::borrow(&skip_list::get_node_nexts(node), l);
                    }
                };
                opt_next_0_score = vector::borrow(&nexts, 0);
            };
            l = l - 1;
        };
    }

    #[test_only]
    fun get_all_socres<V: store>(list: &skip_list::SkipList<V>): vector<u64> {
        let (opt_next_score, scores) = (&skip_list::head(list), vector::empty<u64>());
        while (option_u64::is_some(opt_next_score)) {
            let next_score = option_u64::borrow(opt_next_score);
            let next_node = skip_list::borrow_node(list, next_score);
            let score = skip_list::get_node_score(next_node);
            let nexts = skip_list::get_node_nexts(next_node);
            vector::push_back(&mut scores, score);
            opt_next_score = vector::borrow(&nexts, 0);
        };
        scores
    }

    #[test]
    fun test_new() {
        let ctx = &mut tx_context::dummy();
        let skip_list = skip_list::new<u256>(16, 2, 12345, ctx);
        check_skip_list(&skip_list);
        transfer::public_transfer(skip_list, tx_context::sender(ctx));
    }

    #[test_only]
    fun add_node_for_test<V: store + copy + drop>(list: &mut skip_list::SkipList<V>, size: u64, seed: u64, value: V) {
        let random = random::new(seed);
        let n = 0;
        while (n < size) {
            let score = random::rand_n(&mut random, 1000000);
            if (skip_list::contains(list, score)) {
                continue
            };
            skip_list::insert(list, score, value);
            n = n + 1;
        };
        check_skip_list(list);
    }

    #[test_only]
    fun new_list_for_test<V: store + copy + drop>(
        max_leveL: u64, list_p: u64, size: u64, seed: u64, value: V, ctx: &mut TxContext
    ): skip_list::SkipList<V> {
        let list = skip_list::new<V>(max_leveL, list_p, seed, ctx);
        add_node_for_test(&mut list, size, seed, value);
        list
    }

    #[test]
    fun test_insert() {
        let ctx = &mut tx_context::dummy();
        let list = new_list_for_test<u256>(16, 2, 3000, 1234, 0, ctx);
        transfer::public_transfer(list, tx_context::sender(ctx));
    }

    #[test]
    fun test_insert_bench() {
        let ctx = &mut tx_context::dummy();
        let list = skip_list::new<u256>(16, 2, 100000, ctx);
        let n = 0;
        while (n < 1000) {
            skip_list::insert(&mut list, 0 + n, 0);
            skip_list::insert(&mut list, 1000000 - n, 0);
            skip_list::insert(&mut list, 100000 - n, 0);
            n = n + 1;
        };
        debug::print(&skip_list::level(&list));
        transfer::public_transfer(list, tx_context::sender(ctx));
    }

    struct Item has drop, store {
        n: u64,
        score: u64,
        finded: option_u64::OptionU64
    }

    #[test]
    fun test_find() {
        let ctx = &mut tx_context::dummy();
        let list = new_list_for_test<u256>(16, 2, 1000, 12345, 0, ctx);
        let scores = get_all_socres(&list);

        let length = vector::length(&scores);
        let n = length;
        while (n > 0) {
            let score = *vector::borrow(&scores, n - 1);
            let finded = skip_list::find_prev(&list, score, true);
            assert!((option_u64::is_some(&finded) && (option_u64::borrow(&finded) == score)), 0);
            let finded = skip_list::find_prev(&list, score + 1, true);
            assert!(
                (option_u64::is_some(&finded) && (option_u64::borrow(&finded) == score)) ||
                    (option_u64::is_some(&finded) && (option_u64::borrow(&finded) == score + 1)),
                0
            );

            let finded = skip_list::find_prev(&list, score, false);
            if (n >= 2) {
                assert!(
                    (option_u64::is_some(&finded) && (option_u64::borrow(&finded) == *vector::borrow(&scores, n - 2))),
                    0
                );
            } else {
                assert!(option_u64::is_none(&finded), 0);
            };

            let finded = skip_list::find_next(&list, score, true);
            assert!((option_u64::is_some(&finded) && (option_u64::borrow(&finded) == score)), 0);

            let finded = skip_list::find_next(&list, score - 1, true);
            assert!(
                (option_u64::is_some(&finded) && (option_u64::borrow(&finded) == score)) ||
                    (option_u64::is_some(&finded) && (option_u64::borrow(&finded) == (score - 1))),
                0
            );

            let finded = skip_list::find_next(&list, score, false);
            if (n < length) {
                assert!(
                    (option_u64::is_some(&finded) && (option_u64::borrow(&finded) == *vector::borrow(&scores, n))),
                    0
                );
            } else {
                assert!(option_u64::is_none(&finded), 0);
            };
            n = n - 1;
        };
        transfer::public_transfer(list, tx_context::sender(ctx));
    }

    #[test]
    fun test_find_bench() {
        let ctx = &mut tx_context::dummy();
        let list = new_list_for_test<u256>(16, 2, 1000, 12345, 0, ctx);
        let random = random::new(12345);
        let n = 0;
        while (n < 100) {
            let score = random::rand_n(&mut random, 1000000);
            if ((n % 3) == 0) {
                score = score + 1;
            };
            skip_list::find(&list, score);
            _ = score;
            n = n + 1;
        };
        transfer::public_transfer(list, tx_context::sender(ctx));
    }

    #[test]
    fun test_find_next_bench() {
        let ctx = &mut tx_context::dummy();
        let list = new_list_for_test<u256>(16, 2, 1000, 12345, 0, ctx);
        let n = 0;
        let finded = skip_list::find_next(&list, 99999, true);
        while (n < 1 && option_u64::is_some(&finded)) {
            let node = skip_list::borrow_node(&list, option_u64::borrow(&finded));
            finded = skip_list::next_score(node);
            n = n + 1;
        };
        transfer::public_transfer(list, tx_context::sender(ctx));
    }

    #[test]
    fun test_remove() {
        let ctx = &mut tx_context::dummy();
        let list = new_list_for_test<u256>(16, 2, 1000, 5678, 0, ctx);
        let scores = get_all_socres(&list);
        let (n, length) = (0, vector::length(&scores));
        let start = length / 2;
        while (n <= start) {
            let s1 = start - n;
            let s2 = start + n;
            if (s1 >= 0) {
                skip_list::remove(&mut list, *vector::borrow(&scores, s1));
            };
            if (s2 != s1 && s2 < length) {
                skip_list::remove(&mut list, *vector::borrow(&scores, s2));
            };
            n = n + 1;
        };
        check_skip_list(&list);

        add_node_for_test(&mut list, 2000, 7890, 0);
        let scores = get_all_socres(&list);
        let (n, length) = (0, vector::length(&scores));
        let skip = 0;
        while (n < length) {
            skip_list::remove(&mut list, *vector::borrow(&scores, n));
            skip = skip + 1;
            n = n + skip;
        };
        check_skip_list(&list);

        transfer::public_transfer(list, tx_context::sender(ctx));
    }

    #[test]
    fun test_find_in_empty_list() {
        let ctx = &mut tx_context::dummy();
        let list = skip_list::new<u256>(16, 2, 1234, ctx);
        let opt_score = skip_list::find(&list, 1000);
        assert!(option_u64::is_none(&opt_score), 0);

        let opt_score = skip_list::find_prev(&list, 1000, true);
        assert!(option_u64::is_none(&opt_score), 0);

        let opt_score = skip_list::find_prev(&list, 1000, false);
        assert!(option_u64::is_none(&opt_score), 0);

        let opt_score = skip_list::find_next(&list, 1000, true);
        assert!(option_u64::is_none(&opt_score), 0);

        let opt_score = skip_list::find_next(&list, 1000, false);
        assert!(option_u64::is_none(&opt_score), 0);

        transfer::public_transfer(list, tx_context::sender(ctx));
    }
}
