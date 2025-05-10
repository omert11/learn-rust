#[cfg(test)]
use crate::calculator_v3::main::Expression;

#[test]
fn test_1() {
    let s = Expression::from_str("1");
    assert_eq!(s.eval(), 1.0);
}

#[test]
fn test_2() {
    let s = Expression::from_str("1 + 2 * 3");
    assert_eq!(s.eval(), 7.0);
}

#[test]
fn test_3() {
    let s = Expression::from_str("12 * 2 * 5");
    assert_eq!(s.eval(), 120.0);
}

#[test]
fn test_4() {
    let s = Expression::from_str("12 / 3 * (4 - 3 / 2)");
    assert_eq!(s.eval(), 10.0);
}

#[test]
fn test_5() {
    let s = Expression::from_str("123/123*(88^2)");
    assert_eq!(s.eval(), 7744.0);
}
