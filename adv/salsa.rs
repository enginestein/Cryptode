macro_rules! quarter_round {
    ($v1:expr,$v2:expr,$v3:expr,$v4:expr) => {
        $v2 ^= ($v1.wrapping_add($v4).rotate_left(7));
        $v3 ^= ($v2.wrapping_add($v1).rotate_left(9));
        $v4 ^= ($v3.wrapping_add($v2).rotate_left(13));
        $v1 ^= ($v4.wrapping_add($v3).rotate_left(18));
    };
}

pub fn salsa20(input: &[u32; 16], output: &mut [u32; 16]) {
    output.copy_from_slice(&input[..]);
    for _ in 0..10 {
        quarter_round!(output[0], output[4], output[8], output[12]); 
        quarter_round!(output[5], output[9], output[13], output[1]); 
        quarter_round!(output[10], output[14], output[2], output[6]); 
        quarter_round!(output[15], output[3], output[7], output[11]); 
        quarter_round!(output[0], output[1], output[2], output[3]); 
        quarter_round!(output[5], output[6], output[7], output[4]); 
        quarter_round!(output[10], output[11], output[8], output[9]); 
        quarter_round!(output[15], output[12], output[13], output[14]);
    }
    for (a, &b) in output.iter_mut().zip(input.iter()) {
        *a = a.wrapping_add(b);
    }
}