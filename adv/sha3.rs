const B: usize = 1600;
const W: usize = B / 25;
const L: usize = W.ilog2() as usize;
const U8BITS: usize = u8::BITS as usize;

macro_rules! iterate {
    ( $x:ident, $y:ident, $z:ident => $b:block ) => {
        for $y in 0..5 {
            for $x in 0..5 {
                for $z in 0..W {
                    $b
                }
            }
        }
    };
}

type PadFn = fn(isize, isize) -> Vec<bool>;
type SpongeFn = fn(&[bool]) -> [bool; B];

type State = [[[bool; W]; 5]; 5];

fn state_new() -> State {
    [[[false; W]; 5]; 5]
}

fn state_fill(dest: &mut State, bits: &[bool]) {
    let mut i = 0usize;

    iterate!(x, y, z => {
        if i >= bits.len() { return; }
        dest[x][y][z] = bits[i];
        i += 1;
    });
}

fn state_copy(dest: &mut State, src: &State) {
    iterate!(x, y, z => {
        dest[x][y][z] = src[x][y][z];
    });
}

fn state_dump(state: &State) -> [bool; B] {
    let mut bits = [false; B];

    let mut i = 0usize;

    iterate!(x, y, z => {
        bits[i] = state[x][y][z];
        i += 1;
    });

    bits
}

fn theta(state: &mut State) {
    let mut c = [[false; W]; 5];
    let mut d = [[false; W]; 5];
    for x in 0..5 {
        for z in 0..W {
            c[x][z] = state[x][0][z];

            for y in 1..5 {
                c[x][z] ^= state[x][y][z];
            }
        }
    }

    for x in 0..5 {
        for z in 0..W {
            let x1 = (x as isize - 1).rem_euclid(5) as usize;
            let z2 = (z as isize - 1).rem_euclid(W as isize) as usize;

            d[x][z] = c[x1][z] ^ c[(x + 1) % 5][z2];
        }
    }

    iterate!(x, y, z => {
        state[x][y][z] ^= d[x][z];
    });
}

fn rho(state: &mut State) {
    let mut new_state = state_new();

    for z in 0..W {
        new_state[0][0][z] = state[0][0][z];
    }

    let mut x = 1;
    let mut y = 0;

    for t in 0..=23isize {
        for z in 0..W {
            let z_offset: isize = ((t + 1) * (t + 2)) / 2;
            let new_z = (z as isize - z_offset).rem_euclid(W as isize) as usize;

            new_state[x][y][z] = state[x][y][new_z];
        }

        let old_y = y;
        y = ((2 * x) + (3 * y)) % 5;
        x = old_y;
    }

    state_copy(state, &new_state);
}

fn pi(state: &mut State) {
    let mut new_state = state_new();

    iterate!(x, y, z => {
        new_state[x][y][z] = state[(x + (3 * y)) % 5][x][z];
    });

    state_copy(state, &new_state);
}

fn chi(state: &mut State) {
    let mut new_state = state_new();

    iterate!(x, y, z => {
        new_state[x][y][z] = state[x][y][z] ^ ((state[(x + 1) % 5][y][z] ^ true) & state[(x + 2) % 5][y][z]);
    });

    state_copy(state, &new_state);
}

fn rc(t: u8) -> bool {
    let mut b1: u16;
    let mut b2: u16;
    let mut r: u16 = 0x80;

    for _i in 0..(t % 255) {
        b1 = r >> 8;
        b2 = r & 1;
        r |= (b1 ^ b2) << 8;

        b1 = (r >> 4) & 1;
        r &= 0x1EF; 
        r |= (b1 ^ b2) << 4;

        b1 = (r >> 3) & 1;
        r &= 0x1F7; 
        r |= (b1 ^ b2) << 3;

        b1 = (r >> 2) & 1;
        r &= 0x1FB; 
        r |= (b1 ^ b2) << 2;

        r >>= 1;
    }

    (r >> 7) != 0
}

fn iota(state: &mut State, i_r: u8) {
    let mut rc_arr = [false; W];

    for j in 0..=L {
        rc_arr[(1 << j) - 1] = rc((j as u8) + (7 * i_r));
    }

    for (z, bit) in rc_arr.iter().enumerate() {
        state[0][0][z] ^= *bit;
    }
}

fn rnd(state: &mut State, i_r: u8) {
    theta(state);
    rho(state);
    pi(state);
    chi(state);
    iota(state, i_r);
}

fn keccak_f(bits: &[bool]) -> [bool; B] {
    let n_r = 12 + (2 * L);

    let mut state = state_new();
    state_fill(&mut state, bits);

    for i_r in 0..n_r {
        rnd(&mut state, i_r as u8);
    }

    state_dump(&state)
}

fn pad101(x: isize, m: isize) -> Vec<bool> {
    let mut j = -m - 2;

    while j < 0 {
        j += x;
    }

    j %= x;

    let mut ret = vec![false; (j as usize) + 2];
    *ret.first_mut().unwrap() = true;
    *ret.last_mut().unwrap() = true;

    ret
}

fn sponge(f: SpongeFn, pad: PadFn, r: usize, n: &[bool], d: usize) -> Vec<bool> {
    let mut p = Vec::from(n);
    p.append(&mut pad(r as isize, n.len() as isize));

    assert!(r < B);

    let mut s = [false; B];
    for chunk in p.chunks(r) {
        for (s_i, c_i) in s.iter_mut().zip(chunk) {
            *s_i ^= c_i;
        }

        s = f(&s);
    }

    let mut z = Vec::<bool>::new();
    while z.len() < d {
        z.extend(&s);

        s = f(&s);
    }

    z.truncate(d);
    z
}

fn keccak(c: usize, n: &[bool], d: usize) -> Vec<bool> {
    sponge(keccak_f, pad101, B - c, n, d)
}

fn h2b(h: &[u8], n: usize) -> Vec<bool> {
    let mut bits = Vec::with_capacity(h.len() * U8BITS);

    for byte in h {
        for i in 0..u8::BITS {
            let mask: u8 = 1 << i;

            bits.push((byte & mask) != 0);
        }
    }

    assert!(bits.len() == h.len() * U8BITS);

    bits.truncate(n);
    bits
}

fn b2h(s: &[bool]) -> Vec<u8> {
    let m = if s.len() % U8BITS != 0 {
        (s.len() / 8) + 1
    } else {
        s.len() / 8
    };
    let mut bytes = vec![0u8; m];

    for (i, bit) in s.iter().enumerate() {
        let byte_index = i / U8BITS;
        let mask = (*bit as u8) << (i % U8BITS);

        bytes[byte_index] |= mask;
    }

    bytes
}
macro_rules! sha3 {
    ($name:ident, $n:literal) => {
        pub fn $name(m: &[u8]) -> [u8; ($n / U8BITS)] {
            let mut temp = h2b(m, m.len() * U8BITS);
            temp.append(&mut vec![false, true]);

            temp = keccak($n * 2, &temp, $n);

            let mut ret = [0u8; ($n / U8BITS)];

            let temp = b2h(&temp);
            assert!(temp.len() == $n / U8BITS);

            for (i, byte) in temp.iter().enumerate() {
                ret[i] = *byte;
            }

            ret
        }
    };
}