/// 使用可反驳的模式let-else，可以像普通let一样匹配并绑定周围范围中的变量，
/// 或者当模式不匹配时发散（例如break、return、panic！）。
use std::str::FromStr;

// 使用let else模式
fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}
// match + if let
fn get_count_item2(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("Can't segment count item pair: '{s}'"),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    // 名称绑定的范围是它与 match 或 if let-else 表达式不同的主要因素。
    // 您之前可以通过一些不幸的重复和外部 let 来近似这些模式：
    assert_eq!(get_count_item2("3 chairs"), (3, "chairs"));
}
