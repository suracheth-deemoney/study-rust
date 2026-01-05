fn main() {
    let x = 5;
    // x is actually copied in the below instruction rather than
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
