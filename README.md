# os-summer-of-code-daily


## Format
### 小标题（如果有）
  时间分配， 事件
### 问题

## 7.2 

凌晨 2 小时： 学过一部分 rust， 从 rustlings 开始， 完成了部分练习

## 7.3 
凌晨 2 小时： 完成到 options。 还剩七个文件夹。
### 问题
```rust
let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    // make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(Some(value)) = optional_values_vec.pop() {
        println!("current value: {}", value);
    }
```
该代码中，  optional_values_vec.pop() 出来的不应该是 Option<i8> 类型吗， 为什么使用 Some(Some(value)) 进行匹配， 我以为是 Some(value) 就足够了。
  
 早上 3 小时：完成了 rustlings
 
 ## 7.4 
 晚上一小时： 做了要求的基础练习
 
 ## 7.7
 做了部分 leetcode
 
 ## 7.15
 开始看 RISC-V 手册
关于 ISA： https://zh.wikipedia.org/wiki/%E6%8C%87%E4%BB%A4%E9%9B%86%E6%9E%B6%E6%A7%8B
但是看懂的不多

## 7.16

