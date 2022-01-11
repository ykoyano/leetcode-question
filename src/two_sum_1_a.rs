struct Solution {}

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (index, num) in nums.iter().enumerate() {
            let residual = target - num;

            let index2 = nums
                .iter()
                .position(|&r| r == residual);

            let index3 = match index2 {
                Some(index2) => index2,
                None => continue
            };

            if index == index3 {
                continue;
            }

            return vec![index as i32, index3 as i32];
        }

        return vec![];
    }
}

pub fn execute() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let answer = Solution::two_sum(nums, target);
    println!("{:?}", answer);
}
