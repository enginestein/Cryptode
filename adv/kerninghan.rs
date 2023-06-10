pub fn kerninghan(n: u32) -> i32 {
    let mut count = 0;
    let mut n = n;

    while n > 0 {
        n = n & (n - 1);
        count += 1;
    }

    count
}