use ffi;
use libc::c_ulonglong;
use randombytes::randombytes_into;
pub const SALTBYTES: usize = ffi::crypto_pwhash_scryptsalsa208sha256_SALTBYTES as usize;

pub const HASHEDPASSWORDBYTES: usize = ffi::crypto_pwhash_scryptsalsa208sha256_STRBYTES as usize;

pub const STRPREFIX: &[u8] = ffi::crypto_pwhash_scryptsalsa208sha256_STRPREFIX;

pub const OPSLIMIT_INTERACTIVE: OpsLimit =
    OpsLimit(ffi::crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_INTERACTIVE as usize);

pub const MEMLIMIT_INTERACTIVE: MemLimit =
    MemLimit(ffi::crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_INTERACTIVE as usize);

pub const OPSLIMIT_SENSITIVE: OpsLimit =
    OpsLimit(ffi::crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_SENSITIVE as usize);

pub const MEMLIMIT_SENSITIVE: MemLimit =
    MemLimit(ffi::crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_SENSITIVE as usize);

#[derive(Copy, Clone)]
pub struct OpsLimit(pub usize);

#[derive(Copy, Clone)]
pub struct MemLimit(pub usize);

new_type! {
    public Salt(SALTBYTES);
}

new_type! {
    public HashedPassword(HASHEDPASSWORDBYTES);
}

pub fn gen_salt() -> Salt {
    let mut salt = Salt([0; SALTBYTES]);
    randombytes_into(&mut salt.0);
    salt
}

pub fn derive_key<'a>(
    key: &'a mut [u8],
    passwd: &[u8],
    salt: &Salt,
    OpsLimit(opslimit): OpsLimit,
    MemLimit(memlimit): MemLimit,
) -> Result<&'a [u8], ()> {
    if unsafe {
        ffi::crypto_pwhash_scryptsalsa208sha256(
            key.as_mut_ptr(),
            key.len() as c_ulonglong,
            passwd.as_ptr() as *const _,
            passwd.len() as c_ulonglong,
            salt.0.as_ptr(),
            opslimit as c_ulonglong,
            memlimit,
        )
    } == 0
    {
        Ok(key)
    } else {
        Err(())
    }
}

pub fn derive_key_interactive<'a>(
    key: &'a mut [u8],
    passwd: &[u8],
    salt: &Salt,
) -> Result<&'a [u8], ()> {
    derive_key(
        key,
        passwd,
        salt,
        OPSLIMIT_INTERACTIVE,
        MEMLIMIT_INTERACTIVE,
    )
}

pub fn derive_key_sensitive<'a>(
    key: &'a mut [u8],
    passwd: &[u8],
    salt: &Salt,
) -> Result<&'a [u8], ()> {
    derive_key(key, passwd, salt, OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE)
}

pub fn pwhash(
    passwd: &[u8],
    OpsLimit(opslimit): OpsLimit,
    MemLimit(memlimit): MemLimit,
) -> Result<HashedPassword, ()> {
    let mut hp = HashedPassword([0; HASHEDPASSWORDBYTES]);
    if unsafe {
        ffi::crypto_pwhash_scryptsalsa208sha256_str(
            hp.0.as_mut_ptr() as *mut _,
            passwd.as_ptr() as *const _,
            passwd.len() as c_ulonglong,
            opslimit as c_ulonglong,
            memlimit,
        )
    } == 0
    {
        Ok(hp)
    } else {
        Err(())
    }
}

pub fn pwhash_interactive(passwd: &[u8]) -> Result<HashedPassword, ()> {
    pwhash(passwd, OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE)
}

pub fn pwhash_sensitive(passwd: &[u8]) -> Result<HashedPassword, ()> {
    pwhash(passwd, OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE)
}

pub fn pwhash_verify(hp: &HashedPassword, passwd: &[u8]) -> bool {
    unsafe {
        ffi::crypto_pwhash_scryptsalsa208sha256_str_verify(
            hp.0.as_ptr() as *const _,
            passwd.as_ptr() as *const _,
            passwd.len() as c_ulonglong,
        ) == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_derive_key() {
        let mut kb = [0u8; 32];
        let salt = Salt([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ]);
        let pw = b"Correct Horse Battery Staple";
        let key_expected = [
            0xf1, 0xbb, 0xb8, 0x7c, 0x43, 0x36, 0x5b, 0x03, 0x3b, 0x9a, 0xe8, 0x3e, 0x05, 0xef,
            0xad, 0x25, 0xdb, 0x8d, 0x83, 0xb8, 0x3d, 0xb1, 0xde, 0xe3, 0x6b, 0xdb, 0xf5, 0x4d,
            0xcd, 0x3a, 0x1a, 0x11,
        ];
        let key = derive_key(
            &mut kb,
            pw,
            &salt,
            OPSLIMIT_INTERACTIVE,
            MEMLIMIT_INTERACTIVE,
        )
        .unwrap();
        assert_eq!(key, key_expected);
    }

    #[test]
    fn test_pwhash_verify() {
        use randombytes::randombytes;
        for i in 0..32usize {
            let pw = randombytes(i);
            let pwh = pwhash(&pw, OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE).unwrap();
            assert!(pwhash_verify(&pwh, &pw));
        }
    }

    #[test]
    fn test_pwhash_verify_tamper() {
        use randombytes::randombytes;
        for i in 0..16usize {
            let mut pw = randombytes(i);
            let pwh = pwhash(&pw, OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE).unwrap();
            for j in 0..pw.len() {
                pw[j] ^= 0x20;
                assert!(!pwhash_verify(&pwh, &pw));
                pw[j] ^= 0x20;
            }
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialisation() {
        use randombytes::randombytes;
        use test_utils::round_trip;
        for i in 0..32usize {
            let pw = randombytes(i);
            let pwh = pwhash(&pw, OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE).unwrap();
            let salt = gen_salt();
            round_trip(pwh);
            round_trip(salt);
        }
    }
}
