fn sort<T: Ord>(array: &mut [T]) {
    // 空切片或单元素切片无需排序，直接返回
    if array.len() <= 1 {
        return;
    }
    // 调用快速排序的核心函数
    quick_sort(array, 0, array.len() - 1);
}

/// 快速排序的核心递归函数
/// 参数：array是待排序的切片，low是起始索引，high是结束索引
fn quick_sort<T: Ord>(array: &mut [T], low: usize, high: usize) {
    if low < high {
        // 分区操作，获取基准元素的最终位置
        let pivot_index = partition(array, low, high);

        // 递归排序基准左边的元素（注意处理pivot_index为0的情况，避免usize下溢）
        if pivot_index > 0 {
            quick_sort(array, low, pivot_index - 1);
        }
        // 递归排序基准右边的元素
        quick_sort(array, pivot_index + 1, high);
    }
}

/// 分区函数：将数组分为两部分，返回基准元素的索引
/// 思路：以high位置的元素为基准，将小于等于基准的元素移到左边，大于基准的移到右边
fn partition<T: Ord>(array: &mut [T], low: usize, high: usize) -> usize {
    // 选择最右侧元素作为基准
    let pivot = high;
    // i是小于基准区域的最后一个元素的索引，初始为low-1（空区域）
    let mut i = low - 1;

    // 遍历从low到high-1的元素
    for j in low..high {
        // 如果当前元素小于等于基准，将其加入小于基准的区域
        if array[j] <= array[pivot] {
            i += 1;
            // 交换i和j位置的元素
            array.swap(i, j);
        }
    }

    // 将基准元素放到正确的位置（i+1）
    i += 1;
    array.swap(i, pivot);

    // 返回基准元素的索引
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }

    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
