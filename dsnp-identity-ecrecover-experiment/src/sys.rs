#![no_std]

extern "C" {
    // ############
    // # Math API #
    // ############
    pub fn ecrecover(
        hash_len: u64,
        hash_ptr: u64,
        sig_len: u64,
        sig_ptr: u64,
        v: u64,
        malleability_flag: u64,
        register_id: u64,
    ) -> u64;
    // #############
    // # Alt BN128 #
    // #############
    pub fn alt_bn128_g1_multiexp(value_len: u64, value_ptr: u64, register_id: u64);
    pub fn alt_bn128_g1_sum(value_len: u64, value_ptr: u64, register_id: u64);
    pub fn alt_bn128_pairing_check(value_len: u64, value_ptr: u64) -> u64;
}
