use std::convert::Into;

impl Solution {
  pub fn get_next(pattern: &[u8]) -> Vec<i32> {
    let m = pattern.len(), i = 0;
    let mut next: Vec<i32> = vec![0; m];
    next[0] = -1;
    let j = next[0];
    while i < m {
      if j < 0 || pattern[i] == pattern[j] {
        next[i + 1] = j + 1;
        i += 1; j += 1;
      } else {
        j = next[j];
      }
    }
    return next;
  }
  pub fn kmp(text: &[u8], pattern: &[u8]) -> i32 {
    let m = pattern.len(), n = text.len();
    let mut i: i32 = 0, mut j: i32 = 0;
    let next = get_next(pattern);
    while i < n && j < m {
      if j < 0 || text[i] == pattern[j] {
        i += 1; j += 1;
      } else {
        j = next[j];
      }
    }
    if i == n && j < m { -1 } else { i - m }
  }
  pub fn str_str(haystack: String, needle: String) -> i32 {
    return kmp(haystack.as_bytes(), needle.as_bytes());
  }
}