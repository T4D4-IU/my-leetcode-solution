// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            // 両方NoneならNoneを返す
            (None, None) => None,
            // 一方がNoneなら他方を返す
            (Some(l), None) => Some(l),
            (None, Some(l)) => Some(l),
            // 両方がSomeなら、値を比較して小さい方を選ぶ
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    l1.next = Self::merge_two_lists(l1.next.take(), Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next.take());
                    Some(l2)
                }
            }
        }
    }
}