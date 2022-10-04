pub fn is_match(s: String, p: String) -> bool {
    let str = s.as_bytes();
    let n = str.len();
    let pat = p.as_bytes();
    let m = pat.len();

    let mut dp = vec![vec![false; m + 1]; n + 1];

    dp[0][0] = true;
    for i in 1..=m {
        if pat[i - 1] == '*' as u8 {
            dp[0][i] = true;
        } else {
            break;
        }
    }

    for i in 1..=n {
        for j in 1..=m {
            if str[i - 1] == pat[j - 1] || pat[j - 1] == '?' as u8 {
                dp[i][j] = dp[i - 1][j - 1];
            } else if pat[j - 1] == '*' as u8 {
                dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
            }
        }
    }
    dp[n][m]
}
