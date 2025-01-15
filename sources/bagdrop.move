module bagdrop::bagdrop {
    use sui::{coin, groth16, table::Table};

    public struct Drop<phantom TOKEN> has key, store {
        id: UID,
        allocation: u64,
        root: vector<u8>,
        vault: sui::balance::Balance<TOKEN>,
        // Walrus object where leaves are stored
        leaves: address,
        registry: Table<address, bool>,
    }

    public fun create_drop<TOKEN>(
        allocation: u64,
        root: vector<u8>,
        funds: coin::Coin<TOKEN>,
        leaves_storage: address,
        ctx: &mut TxContext,
    ) {
        let drop = Drop<TOKEN> {
            id: object::new(ctx),
            allocation,
            root,
            leaves: leaves_storage,
            vault: funds.into_balance(),
            registry: sui::table::new(ctx),
        };
        transfer::public_share_object(drop);
    }

    public fun claim<TOKEN>(
        pvk: vector<u8>,
        proof_points: vector<u8>,
        drop: &mut Drop<TOKEN>,
        ctx: &mut TxContext,
    ): coin::Coin<TOKEN> {
        verify_proof(pvk, proof_points, drop.root, &ctx.sender());

        drop.registry.add(ctx.sender(), true);

        let bal = drop.vault.split(drop.allocation);
        coin::from_balance(bal, ctx)
    }

    /// internal

    fun hash_address(addr: &address): vector<u8> {
        let bts = sui::bcs::to_bytes(addr);

        let mut v1: vector<u256> = vector::empty();
        let mut v2: vector<u256> = vector::empty();

        let mut i = 0;
        while (i < 16) {
            let byte = *vector::borrow(&bts, i);
            vector::push_back(&mut v1, (byte as u256));
            i = i + 1;
        };
        while (i < 32) {
            let byte = *vector::borrow(&bts, i);
            vector::push_back(&mut v2, (byte as u256));
            i = i + 1;
        };

        let res1 = 0x2::poseidon::poseidon_bn254(&v1);
        let res2 = 0x2::poseidon::poseidon_bn254(&v2);

        let final_hash = 0x2::poseidon::poseidon_bn254(&vector[res1, res2]);

        sui::bcs::to_bytes(&final_hash)
    }

    fun verify_proof(
        pvk: vector<u8>,
        proof_points: vector<u8>,
        root: vector<u8>,
        sender: &address,
    ) {
        let pvk = groth16::prepare_verifying_key(&groth16::bn254(), &pvk);

        let address_bytes = hash_address(sender);

        let mut public_input_bytes = root;

        public_input_bytes.append(address_bytes);

        let proof_points = groth16::proof_points_from_bytes(proof_points);
        let public_inputs = groth16::public_proof_inputs_from_bytes(public_input_bytes);
        assert!(
            groth16::verify_groth16_proof(&groth16::bn254(), &pvk, &public_inputs, &proof_points),
        );
    }

    /// test

    #[test_only]
    public fun test_proof(
        pvk: vector<u8>,
        proof_points: vector<u8>,
        root: vector<u8>,
        ctx: &mut TxContext,
    ) {
        verify_proof(pvk, proof_points, root, &ctx.sender());
    }
}
