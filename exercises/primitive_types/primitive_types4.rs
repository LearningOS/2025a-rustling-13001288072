#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // 切片语法：&数组[起始索引..结束索引]，左闭右开，1..4 取索引1、2、3的元素
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice);
}
