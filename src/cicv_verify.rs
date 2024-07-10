use crate::exercise::Exercise;
use crate::run::run;
use anyhow::{Context, Result};
use std::time::Instant;
use tokio::task;

pub async fn cicv_verify(exercises: &[Exercise]) -> Result<()> {
    // 并行验证所有练习
    let mut handles = vec![];
    for exercise in exercises {
        let exercise = exercise.clone();
        let handle = task::spawn(async move {
            let start = Instant::now();
            let result = run(&exercise, true).context(format!("Failed to run {}", exercise.name));
            let duration = start.elapsed();
            (exercise.name, result, duration)
        });
        handles.push(handle);
    }

    // 收集所有结果
    let mut results = vec![];
    for handle in handles {
        let result = handle.await.context("Failed to join task")?;
        results.push(result);
    }

    // 生成验证结果输出
    for (name, result, duration) in results {
        match result {
            Ok(_) => println!("Exercise {} passed in {:?}", name, duration),
            Err(err) => println!("Exercise {} failed: {:?}", name, err),
        }
    }

    Ok(())
}
