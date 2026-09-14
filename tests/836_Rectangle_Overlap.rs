pub struct Solution;
impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        if rec1[2] <= rec2[0] {
            return false;
        }
        if rec2[2] <= rec1[0] {
            return false;
        }
        if rec1[3] <= rec2[1] {
            return false;
        }
        if rec2[3] <= rec1[1] {
            return false;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![0, 0, 2, 2]) == true);
        assert!(Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]) == false);
        assert!(Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![2, 2, 3, 3]) == false);
    }
}
