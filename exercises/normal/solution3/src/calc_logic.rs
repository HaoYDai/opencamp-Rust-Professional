pub fn new_birthday_probability(n: u32) -> f64 {
    if n == 0 {
        return 0.0;
    }
    
    // 计算至少有两个人生日相同的概率
    // 使用互补事件: P(至少两个相同) = 1 - P(全部不同)
    
    // 使用浮点数进行计算以保持精度
    let days = 365.0;
    let n = n as f64;
    
    // 计算所有生日都不同的概率
    let mut not_same_prob = 1.0;
    for i in 0..n as u32 {
        not_same_prob *= (days - i as f64) / days;
    }
    
    // 返回至少有两个人生日相同的概率
    // 四舍五入到4位小数
    ((1.0 - not_same_prob) * 10000.0).round() / 10000.0
}
