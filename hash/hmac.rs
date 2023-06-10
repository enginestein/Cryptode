fn hmac(hash_func: &dyn Fn(&[u8]) -> Vec<u8>, key: &[u8], message: &[u8]) -> Vec<u8> {
    const BLOCK_SIZE: usize = 64;

    let mut padded_key = if key.len() <= BLOCK_SIZE {
        key.to_vec()
    } else {
        hash_func(key)
    };
    padded_key.resize(BLOCK_SIZE, 0x00);

    let mut opad = Vec::with_capacity(BLOCK_SIZE);
    let mut ipad = Vec::with_capacity(BLOCK_SIZE);
    for byte in padded_key.iter() {
        opad.push(byte ^ 0x5C);
        ipad.push(byte ^ 0x36);
    }

    let mut inner_hash_input = Vec::with_capacity(ipad.len() + message.len());
    inner_hash_input.extend_from_slice(&ipad);
    inner_hash_input.extend_from_slice(message);
    let inner_hash = hash_func(&inner_hash_input);

    let mut outer_hash_input = Vec::with_capacity(opad.len() + inner_hash.len());
    outer_hash_input.extend_from_slice(&opad);
    outer_hash_input.extend_from_slice(&inner_hash);
    hash_func(&outer_hash_input)
}

