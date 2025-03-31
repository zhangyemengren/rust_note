use std::cmp::Ordering;
fn main() {
    let a = [1, 2, 3, 4, 5];
    // all 全部满足
    assert!(a.iter().all(|&x| x > 0));
    // any 任意一个满足
    assert!(a.iter().any(|&x| x % 2 == 0));
    // find 找到第一个满足条件的元素
    assert_eq!(a.iter().find(|&&x| x % 2 == 0), Some(&2));
    // position 找到第一个满足条件的元素的索引
    assert_eq!(a.iter().position(|&x| x % 2 == 1), Some(0));
    // rposition 找到最后一个满足条件的元素的索引
    assert_eq!(a.iter().rposition(|&x| x % 2 == 1), Some(4));
    // max 最大值
    assert_eq!(a.iter().max(), Some(&5));
    // min 最小值
    assert_eq!(a.iter().min(), Some(&1));
    // max min需实现Ord 所以浮点情况可以reduce处理
    assert_eq!(
        [2.4, f32::NAN, 1.3]
            .into_iter()
            .reduce(f32::max)
            .unwrap_or(0.),
        2.4
    );
    // rev 反转
    assert_eq!(a.into_iter().rev().collect::<Vec<_>>(), vec![5, 4, 3, 2, 1]);
    let a = [-3_i32, 0, 1, 5, -10];
    // max min by key
    assert_eq!(*a.iter().max_by_key(|x| x.abs()).unwrap(), -10);
    // min max by
    assert_eq!(*a.iter().min_by(|x, y| x.cmp(y)).unwrap(), -10);

    // cycle 循环迭代器
    let mut cycle = [1, 2].iter().cycle();
    assert_eq!(cycle.next(), Some(&1));
    assert_eq!(cycle.next(), Some(&2));
    assert_eq!(cycle.next(), Some(&1));
    assert_eq!(cycle.next(), Some(&2));

    // sum 求和
    assert_eq!([1, 2, 3, 4, 5].iter().sum::<i32>(), 15);

    // product 求积
    assert_eq!([1, 2, 3, 4, 5].iter().product::<i32>(), 120);

    // cmp 比较 以下都是字典序比较
    assert_eq!([1, 2, 3].iter().cmp([1, 2, 3].iter()), Ordering::Equal);
    assert_eq!([1, 2, 3].iter().cmp([6].iter()), Ordering::Less);
    assert_eq!([5].iter().cmp([1, 1, 1].iter()), Ordering::Greater);

    // eq 相等
    assert_eq!([1, 2, 3].iter().eq([1, 2, 3].iter()), true);

    // ne 不相等
    assert_eq!([1, 2, 3].iter().ne([1, 2, 4].iter()), true);

    // lt 小于
    assert_eq!([1, 2, 3].iter().lt([1, 2, 4].iter()), true);

    // le 小于等于
    assert_eq!([1, 2, 3].iter().le([1, 2, 3].iter()), true);

    // gt 大于
    assert_eq!([1, 2, 3].iter().gt([1, 2, 2].iter()), true);

    // ge 大于等于
    assert_eq!([1, 2, 3].iter().ge([1, 2, 3].iter()), true);
}
