// component2_test.rs

use component2_lib::Component2;

#[test]
fn test_transform() {
    let comp = Component2::new();
    let result = comp.transform(42);
    assert_eq!(result.original, 42);
    assert_eq!(result.transformed, "Value-0042");
}
