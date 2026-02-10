// component2_test.rs

use component2_dll::Component2;

#[test]
fn test_transform() {
    let comp = Component2::new();
    let result = comp.transform(314);
    assert_eq!(result.original, 314);
    assert_eq!(result.transformed, "Value-0314");
}
