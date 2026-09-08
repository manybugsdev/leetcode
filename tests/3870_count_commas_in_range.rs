pub struct Solution;
impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        let mut ans = 0;
        let mut base = 1000;
        while base <= n {
            ans += n - base + 1;
            base *= 1000;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(Solution::count_commas(1002) == 3);
        assert!(Solution::count_commas(998) == 0);
    }
}
