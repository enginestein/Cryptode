macro_rules! quarter_round {
    ($a:expr,$b:expr,$c:expr,$d:expr) => {
        $a = $a.wrapping_add($b);
        $d = ($d ^ $a).rotate_left(16);
        $c = $c.wrapping_add($d);
        $b = ($b ^ $c).rotate_left(12);
        $a = $a.wrapping_add($b);
        $d = ($d ^ $a).rotate_left(8);
        $c = $c.wrapping_add($d);
        $b = ($b ^ $c).rotate_left(7);
    };
}

#[allow(dead_code)]
pub const C: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

pub fn chacha20(input: &[u32; 16], output: &mut [u32; 16]) {
    output.copy_from_slice(&input[..]);
    for _ in 0..10 {
        quarter_round!(output[0], output[4], output[8], output[12]); 
        quarter_round!(output[1], output[5], output[9], output[13]); 
        quarter_round!(output[2], output[6], output[10], output[14]); 
        quarter_round!(output[3], output[7], output[11], output[15]);
        quarter_round!(output[0], output[5], output[10], output[15]); 
        quarter_round!(output[1], output[6], output[11], output[12]); 
        quarter_round!(output[2], output[7], output[8], output[13]); 
        quarter_round!(output[3], output[4], output[9], output[14]); 
    }
    for (a, &b) in output.iter_mut().zip(input.iter()) {
        *a = a.wrapping_add(b);
    }
}