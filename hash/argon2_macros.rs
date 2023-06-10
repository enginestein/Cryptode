macro_rules! argon2_module (($pwhash_name:ident,
                           $pwhash_str_name:ident,
                           $pwhash_str_verify_name:ident,
                           $saltbytes:expr,
                           $hashedpasswordbytes:expr,
                           $strprefix:expr,
                           $opslimit_interative:expr,
                           $opslimit_moderate:expr,
                           $opslimit_sensitive:expr,
                           $memlimit_interative:expr,
                           $memlimit_moderate:expr,
                           $memlimit_sensitive:expr,
                           $variant:expr) => (

use libc::{c_int, c_ulonglong};
use randombytes::randombytes_into;

pub const SALTBYTES: usize = $saltbytes;
pub const HASHEDPASSWORDBYTES: usize = $hashedpasswordbytes;
pub const STRPREFIX: &'static [u8] = $strprefix;
pub const OPSLIMIT_INTERACTIVE: OpsLimit = OpsLimit($opslimit_interative);
pub const MEMLIMIT_INTERACTIVE: MemLimit = MemLimit($memlimit_interative);
pub const OPSLIMIT_MODERATE: OpsLimit = OpsLimit($opslimit_moderate);
pub const MEMLIMIT_MODERATE: MemLimit = MemLimit($memlimit_moderate);
pub const OPSLIMIT_SENSITIVE: OpsLimit = OpsLimit($opslimit_sensitive);
pub const MEMLIMIT_SENSITIVE: MemLimit = MemLimit($memlimit_sensitive);
pub const VARIANT: u32 = $variant;
#[derive(Copy, Clone, Debug)]
pub struct OpsLimit(pub usize);
#[derive(Copy, Clone, Debug)]
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
    &Salt(ref sb): &Salt,
    OpsLimit(opslimit): OpsLimit,
    MemLimit(memlimit): MemLimit,
) -> Result<&'a [u8], ()> {

    let res = unsafe {
        $pwhash_name(
            key.as_mut_ptr(),
            key.len() as c_ulonglong,
            passwd.as_ptr() as *const _,
            passwd.len() as c_ulonglong,
            sb as *const _,
            opslimit as c_ulonglong,
            memlimit,
            VARIANT as c_int)
    };

    match res {
        0 => Ok(key),
        _ => Err(()),
    }
}


pub fn pwhash(
    passwd: &[u8],
    OpsLimit(opslimit): OpsLimit,
    MemLimit(memlimit): MemLimit,
) -> Result<HashedPassword, ()> {
    let mut out = HashedPassword([0; HASHEDPASSWORDBYTES]);
    let res = unsafe {
        $pwhash_str_name(
            out.0.as_mut_ptr() as *mut _,
            passwd.as_ptr() as *const _,
            passwd.len() as c_ulonglong,
            opslimit as c_ulonglong,
            memlimit)
    };

    match res {
        0 => Ok(out),
        _ => Err(()),
    }
}

pub fn pwhash_verify(hp: &HashedPassword, passwd: &[u8]) -> bool {
    let res = unsafe {
        $pwhash_str_verify_name(
            hp.0.as_ptr() as *const _,
            passwd.as_ptr() as *const _,
            passwd.len() as c_ulonglong)
    };

    res == 0
}

));
