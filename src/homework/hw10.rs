fn is_palindrome(n: u32) -> bool {
    let original = n.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}

#[test]
fn test() {
    let test_cases = [
        (121, true),
        (12321, true),
        (123, false),
        (1, true),
        (10, false),
        (1221, true),
        (123321, true),
        (100, false),
    ];

    for (num, expected) in test_cases {
        assert_eq!(is_palindrome(num), expected);
    }
}
