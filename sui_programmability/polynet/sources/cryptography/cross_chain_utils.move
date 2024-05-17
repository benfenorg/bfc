module polynet::cross_chain_utils {
    use std::hash;
    use std::vector;
    use std::option;
    use polynet::secp256k1;
    use polynet::utils as putil;

    #[test_only]
    use sui::event;

    const MERKLE_PROOF_NODE_LEN: u64 = 33;
    const POLYCHAIN_SIGNATURE_LEN: u64 = 65;
    const APTOS_SIGNATURE_LEN: u64 = 65;

    const EINVALID_POSITION: u64 = 2003;
    const EROOT_NOT_MATCH: u64 = 2004;
    const ENOT_ENOUGH_SIG: u64 = 2005;

    // struct Header has copy, drop {
    //     version: u64,
    //     chainId: u64,
    //     timestamp: u64,
    //     height: u64,
    //     consensusData: u64,
    //     prevBlockHash: vector<u8>,
    //     transactionsRoot: vector<u8>,
    //     crossStatesRoot: vector<u8>,
    //     blockRoot: vector<u8>,
    //     consensusPayload: vector<u8>,
    //     nextBookkeeper: vector<u8>
    // }

    // struct ToMerkleValue has copy, drop {
    //     txHash: vector<u8>,
    //     fromChainID: u64,
    //     makeTxParam: TxParam
    // }

    // struct TxParam has copy, drop {
    //     txHash: vector<u8>,
    //     crossChainId: vector<u8>,
    //     fromContract: vector<u8>,
    //     toChainId: u64,
    //     toContract: vector<u8>,
    //     method: vector<u8>,
    //     args: vector<u8>
    // }

    public fun merkle_prove(auditPath: &vector<u8>, root: &vector<u8>): vector<u8> {
        let offset: u64 = 0;
        let value: vector<u8>;
        (value, offset) = polynet::zero_copy_source::next_var_bytes(auditPath, offset);
        let hash: vector<u8> = hash_leaf(&value);
        let size: u64 = (vector::length(auditPath) - offset) / MERKLE_PROOF_NODE_LEN;
        let nodeHash: vector<u8>;
        let index: u64 = 0;
        let pos: u8;
        while (index < size) {
            (pos, offset) = polynet::zero_copy_source::next_byte(auditPath, offset);
            (nodeHash, offset) = polynet::zero_copy_source::next_hash(auditPath, offset);
            hash = if (pos == 0) {
                hash_children(&nodeHash, &hash)
            } else if (pos == 1) {
                hash_children(&hash, &nodeHash)
            } else {
                abort EINVALID_POSITION
            };
            index = index + 1;
        };
        assert!(hash == *root, EROOT_NOT_MATCH);
        return value
    }

    fun hash_leaf(data: &vector<u8>): vector<u8> {
        let data_copy = vector<u8>[0x00];
        vector::append(&mut data_copy, *data);
        return hash::sha2_256(data_copy)
    }

    fun hash_children(l: &vector<u8>, r: &vector<u8>): vector<u8> {
        let data = vector<u8>[0x01];
        vector::append(&mut data, *l);
        vector::append(&mut data, *r);
        return hash::sha2_256(data)
    }

    public  fun verify_sig(rawHeader: &vector<u8>, sigList: &vector<u8>, keepers: &vector<vector<u8>>, threshold: u64): bool  {
        let headerHash = get_header_hash(rawHeader);
        let sigCount = vector::length<u8>(sigList)/POLYCHAIN_SIGNATURE_LEN;
        let signers = vector::empty<vector<u8>>();
        let recovery_id: u8;
        let sig: secp256k1::ECDSASignature;
        let index: u64 = 0;
        while (index < sigCount) {
            sig = secp256k1::ecdsa_signature_from_bytes(putil::slice<u8>(sigList, index*POLYCHAIN_SIGNATURE_LEN, APTOS_SIGNATURE_LEN));
            recovery_id = 0;  //*vector::borrow<u8>(sigList, index*POLYCHAIN_SIGNATURE_LEN + APTOS_SIGNATURE_LEN);
            let signer_opt = secp256k1::ecdsa_recover(headerHash, recovery_id, &sig);
            if (option::is_none(&signer_opt)) {
                continue
            };
            let the_signer = secp256k1::ecdsa_raw_public_key_to_bytes(&option::extract(&mut signer_opt));
            vector::push_back<vector<u8>>(&mut signers, the_signer);
            index = index + 1;
        };
        return contain_addresses(keepers, &signers, threshold)
    }

    #[test_only]
    struct VerifySigEvent has store, drop, copy {
        length: u64,
        headHash: vector<u8>,
        sig: vector<u8>,
        sigList: vector<u8>,
    }

    #[test_only]
    public entry  fun verify_sig_test(rawHeader: vector<u8>, sigList: vector<u8>, keepers: vector<vector<u8>>, threshold: u64)  {
        let headerHash = get_header_hash(&rawHeader);
        let sigCount = vector::length<u8>(&sigList)/POLYCHAIN_SIGNATURE_LEN;
        let signers = vector::empty<vector<u8>>();
        let recovery_id: u8;
        let sig: secp256k1::ECDSASignature;
        let index: u64 = 0;
        while (index < sigCount) {
            let sigByte = putil::slice<u8>(&sigList, index*POLYCHAIN_SIGNATURE_LEN, APTOS_SIGNATURE_LEN);
            sig = secp256k1::ecdsa_signature_from_bytes(sigByte);
            recovery_id =  0; //*vector::borrow<u8>(&sigList, index*POLYCHAIN_SIGNATURE_LEN + APTOS_SIGNATURE_LEN);

            event::emit(
                VerifySigEvent{
                    length: sigCount,
                    headHash: headerHash,
                    sig: sigByte,
                    sigList: sigList,

                }
            );

            let signer_opt = secp256k1::ecdsa_recover(headerHash, recovery_id, &sig);
            if (option::is_none(&signer_opt)) {
                    abort EINVALID_POSITION
            };
            let the_signer = secp256k1::ecdsa_raw_public_key_to_bytes(&option::extract(&mut signer_opt));
            vector::push_back<vector<u8>>(&mut signers, the_signer);
            index = index + 1;
        };
        let result = contain_addresses(&keepers, &signers, threshold);
        if (!result) {
            abort ENOT_ENOUGH_SIG
        }
    }


    fun contain_addresses(keepers: &vector<vector<u8>>, signers: &vector<vector<u8>>, threshold: u64): bool {
        let keepers_copy = *keepers;
        let cnt: u64 = 0; 
        while (!vector::is_empty<vector<u8>>(&keepers_copy)) {
            let s = vector::pop_back<vector<u8>>(&mut keepers_copy);
            if (vector::contains<vector<u8>>(signers, &s)) {
                cnt = cnt + 1;
            };
        };
        return cnt >= threshold
    }

    public fun deserialize_merkle_value(valueBs: &vector<u8>): (
        vector<u8>,
        u64,
        vector<u8>,
        vector<u8>,
        vector<u8>,
        u64,
        vector<u8>,
        vector<u8>,
        vector<u8>) 
    {
        let txHash: vector<u8>;
        let fromChainID: u64;
        let txParam_txHash: vector<u8>;
        let txParam_crossChainId: vector<u8>;
        let txParam_fromContract: vector<u8>;
        let txParam_toChainId: u64;
        let txParam_toContract: vector<u8>;
        let txParam_method: vector<u8>;
        let txParam_args: vector<u8>;
        let offset: u64 = 0;

        (txHash, offset) = polynet::zero_copy_source::next_var_bytes(valueBs, offset);
        (fromChainID, offset) = polynet::zero_copy_source::next_u64(valueBs, offset);

        (txParam_txHash, offset) = polynet::zero_copy_source::next_var_bytes(valueBs, offset);
        (txParam_crossChainId, offset) = polynet::zero_copy_source::next_var_bytes(valueBs, offset);
        (txParam_fromContract, offset) = polynet::zero_copy_source::next_var_bytes(valueBs, offset);
        (txParam_toChainId, offset) = polynet::zero_copy_source::next_u64(valueBs, offset);
        (txParam_toContract, offset) = polynet::zero_copy_source::next_var_bytes(valueBs, offset);
        (txParam_method, offset) = polynet::zero_copy_source::next_var_bytes(valueBs, offset);
        (txParam_args, _) = polynet::zero_copy_source::next_var_bytes(valueBs, offset);

        return (
            txHash,
            fromChainID,
            txParam_txHash,
            txParam_crossChainId,
            txParam_fromContract,
            txParam_toChainId,
            txParam_toContract,
            txParam_method,
            txParam_args
        )
    }

    public fun deserialize_header(headerBs : &vector<u8>): (
        u64,
        u64,
        u64,
        u64,
        u64,
        vector<u8>,
        vector<u8>,
        vector<u8>,
        vector<u8>,
        vector<u8>,
        vector<u8>)
    {
        let version: u64;
        let chainId: u64;
        let timestamp: u64;
        let height: u64;
        let consensusData: u64;
        let prevBlockHash: vector<u8>;
        let transactionsRoot: vector<u8>;
        let crossStatesRoot: vector<u8>;
        let blockRoot: vector<u8>;
        let consensusPayload: vector<u8>;
        let nextBookkeeper: vector<u8>;
        let offset: u64 = 0;

        (version, offset) = polynet::zero_copy_source::next_u32(headerBs , offset);
        (chainId, offset) = polynet::zero_copy_source::next_u64(headerBs , offset);
        (prevBlockHash, offset) = polynet::zero_copy_source::next_hash(headerBs , offset);
        (transactionsRoot, offset) = polynet::zero_copy_source::next_hash(headerBs , offset);
        (crossStatesRoot, offset) = polynet::zero_copy_source::next_hash(headerBs , offset);
        (blockRoot, offset) = polynet::zero_copy_source::next_hash(headerBs , offset);
        (timestamp, offset) = polynet::zero_copy_source::next_u32(headerBs , offset);
        (height, offset) = polynet::zero_copy_source::next_u32(headerBs , offset);
        (consensusData, offset) = polynet::zero_copy_source::next_u64(headerBs , offset);
        (consensusPayload, offset) = polynet::zero_copy_source::next_var_bytes(headerBs , offset);
        (nextBookkeeper, _) = polynet::zero_copy_source::next_bytes20(headerBs , offset);
        
        return (
            version,
            chainId,
            timestamp,
            height,
            consensusData,
            prevBlockHash,
            transactionsRoot,
            crossStatesRoot,
            blockRoot,
            consensusPayload,
            nextBookkeeper
        )
    }

    public fun get_header_hash(rawHeader: &vector<u8>): vector<u8> {
        return hash::sha2_256(hash::sha2_256(*rawHeader))
    }
}

/*

    struct Header {
        uint32 version;
        uint64 chainId;
        uint32 timestamp;
        uint32 height;
        uint64 consensusData;
        bytes32 prevBlockHash;
        bytes32 transactionsRoot;
        bytes32 crossStatesRoot;
        bytes32 blockRoot;
        bytes consensusPayload;
        bytes20 nextBookkeeper;
    }

    struct ToMerkleValue {
        bytes  txHash;  // cross chain txhash
        uint64 fromChainID;
        TxParam makeTxParam;
    }

    struct TxParam {
        bytes txHash; //  source chain txhash
        bytes crossChainId;
        bytes fromContract;
        uint64 toChainId;
        bytes toContract;
        bytes method;
        bytes args;
    }

*/