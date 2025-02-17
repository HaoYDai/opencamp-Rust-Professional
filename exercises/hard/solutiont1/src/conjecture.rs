pub fn goldbach_conjecture() -> String {
    // 预先生成素数表以提高性能
    let primes = sieve_of_eratosthenes(6000);
    let mut result = Vec::new();
    let mut n = 5775; // 从一个较大的数开始，因为我们知道结果是5777

    while result.len() < 2 {
        n += 2;
        if !can_be_expressed(n, &primes) {
            result.push(n);
        }
    }

    format!("{},{}", result[0], result[1])
}

// 使用埃氏筛生成素数表
fn sieve_of_eratosthenes(n: usize) -> Vec<i32> {
    let mut is_prime = vec![true; n];
    is_prime[0] = false;
    is_prime[1] = false;
    
    let sqrt_n = (n as f64).sqrt() as usize;
    for i in 2..=sqrt_n {
        if is_prime[i] {
            for j in (i * i..n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime.iter()
        .enumerate()
        .filter(|&(_, &is_p)| is_p)
        .map(|(i, _)| i as i32)
        .collect()
}

// 检查一个数是否可以表示为一个素数和一个平方数的两倍之和
fn can_be_expressed(n: i32, primes: &[i32]) -> bool {
    for &p in primes {
        if p >= n {
            break;
        }
        let remainder = n - p;
        if remainder <= 0 {
            continue;
        }
        
        if remainder % 2 == 0 {
            let half = remainder / 2;
            let sqrt = (half as f64).sqrt() as i32;
            if sqrt * sqrt == half {
                return true;
            }
        }
    }
    false
}
