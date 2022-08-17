fn closest(mut input: i32) -> i32 {
    let mut extracted = input & !(input - 1);
    return match extracted {
        1 => input + 1,
        _ => {
            input &= input - 1;
            extracted >>= 1;
            input ^= extracted;
            input
        }
    };
}

#[test]
fn test_closest() {
    assert_eq!(closest(16), 8);
    assert_eq!(closest(14), 13);
    assert_eq!(closest(14), 13);
    assert_eq!(closest(1), 2);
    assert_eq!(closest(3), 4);
    assert_eq!(closest(5), 6);
}
