/// try系列是一些加强 闭包返回Try类型
fn main() {
    use std::ops::ControlFlow;
    // try_fold
    let a = [1, 2, 3, 4];
    let sum = a.iter().try_fold(0i8, |acc, &x| acc.checked_add(x));
    let sum2 = a
        .iter()
        .try_fold(0i8, |acc, &x| if x > 2 { None } else { Some(acc + x) });
    let sum3: Result<i8, Box<dyn std::error::Error>> =
        a.iter().try_fold(0i8, |acc, &x| Ok(acc + x));
    println!("sum: {:?}", sum);
    println!("sum2: {:?}", sum2);
    println!("sum3: {:?}", sum3);
    // 在try中可使用控制流提前返回
    let sum4 = a.iter().try_fold(0i8, |acc, &x| {
        if x > 2 {
            ControlFlow::Break(x)
        } else {
            ControlFlow::Continue(acc + x)
        }
    });
    println!("sum4: {:?}", sum4.break_value().unwrap());
    // try_for_each
    let res = (0..10).try_for_each(|_x| ControlFlow::Continue::<i32>(()));
    println!("res: {:?}", res.continue_value().unwrap());

    let res2 = (0..10).try_for_each(|x| if x > 5 { Err(x) } else { Ok(()) });
    println!("res2: {:?}", res2);
}
