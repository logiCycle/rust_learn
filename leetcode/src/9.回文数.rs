/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 *
 * https://leetcode-cn.com/problems/palindrome-number/description/
 *
 * algorithms
 * Easy (58.06%)
 * Likes:    1742
 * Dislikes: 0
 * Total Accepted:    855.7K
 * Total Submissions: 1.5M
 * Testcase Example:  '121'
 *
 * 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
 *
 * 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。例如，121 是回文，而 123 不是。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：x = 121
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：x = -121
 * 输出：false
 * 解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
 *
 *
 * 示例 3：
 *
 *
 * 输入：x = 10
 * 输出：false
 * 解释：从右向左读, 为 01 。因此它不是一个回文数。
 *
 *
 * 示例 4：
 *
 *
 * 输入：x = -101
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * -2^31
 *
 *
 *
 *
 * 进阶：你能不将整数转为字符串来解决这个问题吗？
 *
 */

// @lc code=start
// use std::collections::VecDeque;
impl Solution {
    // pub fn is_palindrome(x: i32) -> bool {
    //     if x < 0 {
    //         return false;
    //     }
    //     let mut queue = VecDeque::new();
    //     let mut tmp = x;
    //     while tmp != 0 {
    //         queue.push_back(tmp % 10);
    //         tmp = tmp / 10;
    //     }
    //     let len = queue.len();
    //     let mut reverse_x = 0;
    //     for i in 0..len {
    //         if let Some(v) = queue.pop_front(){
    //            reverse_x = reverse_x + v * 10i32.pow((len - (i + 1)) as u32);
    //         }
    //     }
    //     if reverse_x == x {true} else {false}
    // }
    // pub fn is_palindrome(x: i32) -> bool {
    //     if x < 0 {
    //         return false;
    //     }
    //     if x < 10 {
    //         return true;
    //     }
    //     if x % 10 == 0 {
    //         return false;
    //     }
    //     let mut post = 0;
    //     let mut pre = x;
    //     loop {
    //         if pre <= post {
    //             break;
    //         }
    //         post = post * 10 + pre % 10;
    //         pre = pre / 10;
    //     }
    //     //奇数位时后半比前半多一位
    //     if post == pre || post / 10 == pre {true} else {false}
    // }
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string();
        let rev_str = x.to_string().chars().rev().collect::<String>();
        if x_str == rev_str {true} else {false}
    }
}
// @lc code=end
