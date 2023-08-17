/// 与 if let 类似，while let 可以使尴尬的匹配序列更容易被容忍。考虑以下递增 i 的序列：

fn main() {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    loop {
        match optional {
            // If `optional` destructures, evaluate the block.
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ 需要3次缩进！
            },
            // Quit the loop when the destructure fails:
            _ => { break; }
            // ^ Why should this be required? There must be a better way!
        }
    }
    // 使用 while let 可以使这个序列变得更好：
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 向右漂移较少且不需要
        // 明确处理失败的情况。
    }
    // ^ `if let` 有额外的可选 `else`/`else if` 子句。
    // `while let` 没有这些。
}