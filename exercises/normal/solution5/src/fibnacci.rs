pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold < 2 {
        return 0;
    }

    let mut sum = 2; // 初始化和为2（第一个和第二个斐波那契数之和）
    let mut prev = 1;
    let mut curr = 1;

    while curr < threshold {
        let next = prev + curr;
        prev = curr;
        curr = next;

        // 如果当前数是奇数且小于阈值，加入到和中
        if curr < threshold && curr % 2 == 1 {
            sum += curr;
        }
    }

    sum
}
