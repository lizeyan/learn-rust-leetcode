// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

trait Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) ->
    Option<Box<ListNode>>;
}

struct SolutionImpl;
impl Solution for SolutionImpl {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        let mut head = None;
        let mut residual = 0;
        while l1.is_some() || l2.is_some() || residual > 0 {
            let mut sum = 0;
            if l1.is_some() {
                sum += l1.as_ref().unwrap().val;
                l1 = l1.unwrap().next.as_ref();
            }
            if l2.is_some() {
                sum += l2.as_ref().unwrap().val;
                l2 = l2.unwrap().next.as_ref();
            }
            sum += residual;
            residual = sum / 10;
            sum = sum % 10;
            head = Some(Box::new(ListNode { val: sum, next: head }));
        }
        reverse_list(head)
    }
}


fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut current) = head {
        head = current.next;
        current.next = prev;
        prev = Some(current);
    }
    prev
}


#[cfg(test)]
mod tests {
    use super::*;

    // 定义一个宏来创建链表
    macro_rules! list {
    ( $($val:expr),* ) => {
        {
            let mut head = None;
            let mut current = &mut head;
            $(
                let node = Box::new(ListNode::new($val));
                *current = Some(node);
                current = &mut current.as_mut().unwrap().next;
            )*
            head
        }
    };
}

    // 定义一个宏来比较两个链表是否相等
    macro_rules! assert_list_eq {
        ($l1:expr, $l2:expr) => {
            assert_eq!(list_to_vec($l1), list_to_vec($l2));
        };
    }

    // 将链表转换为 Vec 以便于比较
    fn list_to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = list {
            vec.push(node.val);
            list = node.next;
        }
        vec
    }

    #[test]
    fn test_add_two_numbers() {
        // Test case 1: [2,4,3] + [5,6,4] = [7,0,8]
        assert_list_eq!(
            SolutionImpl::add_two_numbers(
                list![2,4,3],
                list![5,6,4]
            ),
            list![7,0,8]
        );

        // Test case 2: [0] + [0] = [0]
        assert_list_eq!(
            SolutionImpl::add_two_numbers(
                list![0],
                list![0]
            ),
            list![0]
        );

        // Test case 3: [9,9,9,9,9,9,9] + [9,9,9,9] = [8,9,9,9,0,0,0,1]
        assert_list_eq!(
            SolutionImpl::add_two_numbers(
                list![9,9,9,9,9,9,9],
                list![9,9,9,9]
            ),
            list![8,9,9,9,0,0,0,1]
        );

        // Test case 4: [1] + [9,9,9,9,9,9,9,9,9,9] = [0,0,0,0,0,0,0,0,0,0,1]
        assert_list_eq!(
            SolutionImpl::add_two_numbers(
                list![1],
                list![9,9,9,9,9,9,9,9,9,9]
            ),
            list![0,0,0,0,0,0,0,0,0,0,1]
        );
    }
}