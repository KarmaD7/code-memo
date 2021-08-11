use std::convert::TryInto;

impl Solution {
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }
    let mut i = 0;
    let mut new_len = 1;
    while i + 1 < len {
      let mut j = i + 1;
      while j < len && nums[j] == nums[i]{
        j += 1;
      }
      if j < len {
        nums[new_len] = nums[j];
        new_len += 1;
      }
      i = j;
    }
    new_len.try_into().unwrap()
  }
}