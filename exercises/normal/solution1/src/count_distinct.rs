pub fn new_count_distinct(input_str: &str) -> usize {
    input_str
        .split(',')  // 按逗号分割字符串
        .collect::<std::collections::HashSet<_>>()  // 收集到 HashSet 中自动去重
        .len()  // 获取 HashSet 的大小，即不重复元素的数量
}
