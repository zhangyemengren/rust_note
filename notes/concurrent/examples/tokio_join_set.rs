use tokio::task::JoinSet;
#[tokio::main]
async fn main() {
    //这段代码展示了如何使用 Tokio 库中的  JoinSet  来并发地运行多个异步任务，并收集它们的结果。
    // 使用  for  循环生成 10 个异步任务，每个任务简单地返回其索引  i。这些任务被添加到  JoinSet  中进行管理。
    // 创建一个布尔数组  seen，用于跟踪每个任务的结果是否已被处理。使用  while let  循环异步地等待并收集  JoinSet  中下一个完成的任务结果。res.unwrap()  提取任务的结果，并将对应索引的  seen  值设置为  true。
    // 使用  for  循环验证所有任务的结果都已被处理。如果所有任务都成功完成并且结果被正确收集，程序将正常结束；否则，断言将失败并引发错误。
    let mut join_set = JoinSet::new();
    
    for i in 0..10 {
        join_set.spawn(async move {
            i
        });
    }
    
    let mut seen = [false; 10];
    while let Some(res) = join_set.join_next().await {
        let idx = res.unwrap();
        seen[idx] = true;
    }
    
    for i in 0..10 {
        assert!(seen[i]);
    }
}