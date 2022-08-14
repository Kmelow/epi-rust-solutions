pub fn reverse(x: u8) -> u8 {
    let cache = vec![0, 2, 1, 3];

    let i0 = ((x & 0xc0) >> 6) & 0x3;
    let i1 = ((x & 0x30) >> 4) & 0x3;
    let i2 = ((x & 0x0c) >> 2) & 0x3;
    let i3 = x & 0x03;

    cache[i0 as usize] | cache[i1 as usize] << 2 | cache[i2 as usize] << 4 | cache[i3 as usize] << 6
}

#[test]
fn reverse_test() {
    assert_eq!(reverse(0), 0);
    assert_eq!(reverse(1), 128);
    assert_eq!(reverse(240), 15);
    assert_eq!(reverse(160), 5);
    assert_eq!(reverse(100), 38);
}
