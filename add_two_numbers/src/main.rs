// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {

    pub fn add_two_numbers(self, l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) 
      -> Option<Box<ListNode>> {
        
        let mut l1 = l1;
        let mut l2 = l2;

        // let l1_val = l1.map_or_err(|v| v.val.unwrap());
        let mut head = None;
        let mut tail = &mut head;
        let carry = 0;
        let mut carry_after = 0;
        while  l1.is_some() || l2.is_some() || carry_after > 0 {
          let l1_sum = l1.as_ref().map_or(0, |v| v.val);
          let l2_sum = l2.as_ref().map_or(0, |v| v.val);
          let sum = l1_sum + l2_sum + carry;
          let carry = sum / 10;
          let remain = sum % 10;
          let new_node = Box::new(ListNode::new(remain  + carry_after));
          carry_after=carry;
          *tail = Some(new_node);
          tail = &mut tail.as_mut().unwrap().next;
          l1 = l1.and_then(|v| v.next);
          l2 = l2.and_then(|v| v.next);
        
        }
        head

    }

 

    }


fn main() {
  let li1 = Some(Box::new(ListNode{
        val : 2, 
        next : Some(Box::new(ListNode {
          val : 4, 
          next: Some(Box::new(ListNode {
            val : 3, 
            next : None
      }))
    }))
  }));
  let li2 = Some(Box::new(ListNode{
    val : 5, 
    next : Some(Box::new(ListNode {
      val : 6, 
      next: Some(Box::new(ListNode {
        val : 4, 
        next : None
  }))
}))
}));

    let v = Solution.add_two_numbers(li1, li2);
    println!("{:?}", v)
}




// impl Solution {
//   pub fn add_two_numbers(
//       self, 
//       mut l1: Option<Box<ListNode>>, 
//       mut l2: Option<Box<ListNode>>
//   ) -> Option<Box<ListNode>> {
//       let mut head = None;
//       let mut tail = &mut head;
//       let mut carry = 0;

//       while l1.is_some() || l2.is_some() || carry > 0 {
//           let sum = l1.as_ref().map_or(0, |n| n.val) + l2.as_ref().map_or(0, |n| n.val) + carry;
//           let new_node = Box::new(ListNode::new(sum % 10));
//           carry = sum / 10;

//           *tail = Some(new_node);
//           tail = &mut tail.as_mut().unwrap().next;  // safely advance the tail

//           l1 = l1.and_then(|n| n.next);  // move to the next node if it exists
//           l2 = l2.and_then(|n| n.next);
//       }

//       head
//   }
// }
