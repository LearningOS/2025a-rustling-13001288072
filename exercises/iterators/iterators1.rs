fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // Step 1: 获取 Vec 的迭代器（iter() 生成不可变引用的迭代器）
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    // Step 2: 迭代器下一个元素是 "custard apple" 的引用
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    // Step 3: 迭代器下一个元素是 "peach" 的引用
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    // Step 4: 迭代器遍历完所有元素后返回 None
    assert_eq!(my_iterable_fav_fruits.next(), None);
}
