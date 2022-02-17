#![cfg(test)]

#[test]
fn always_true() {
    use leafwing_2d::utils::returns_true;

    assert!(returns_true());
}
