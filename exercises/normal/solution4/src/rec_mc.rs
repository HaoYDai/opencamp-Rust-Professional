pub fn dp_rec_mc(amount: u32) -> u32 {
    if amount == 0 {
        return 0;
    }

    // 可用的硬币面值
    const COINS: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
    
    // dp数组，dp[i]表示金额i需要的最少硬币数
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0;

    // 对每个金额进行计算
    for i in 1..=amount {
        // 尝试每个硬币面值
        for &coin in COINS.iter() {
            if coin <= i {
                // 如果dp[i - coin]有解，则尝试使用当前硬币
                if dp[(i - coin) as usize] != u32::MAX {
                    dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
                }
            }
        }
    }

    // 如果没有解，返回0（根据测试用例的要求）
    if dp[amount as usize] == u32::MAX {
        0
    } else {
        dp[amount as usize]
    }
}
