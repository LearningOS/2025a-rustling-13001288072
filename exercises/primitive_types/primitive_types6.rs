#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // 元组索引从0开始，第二个元素的索引是1
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
