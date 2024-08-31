# Definition for singly-linked list.
class ListNode(object):
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution(object):
    def addTwoNumbers(self, l1, l2):
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtype: ListNode
        """
        first_turn = True
        aft_aft = 0
        firstrt = 0
        while True:
            if (l1 or l2):
                # firstl1, firstl2 = l1.val, l2.val
                l1_val, l2_val = l1.val if l1 else 0 , l2.val if l2 else 0
                aft, rem = divmod((l1_val + l2_val +aft_aft), 10)

                if first_turn:
                    rt = ListNode(rem )
                    firstrt = rt
                    first_turn = False
                else:
                    rt.next = ListNode(rem )
                    rt = rt.next
                
                l1, l2 = l1.next if l1 else None, l2.next if l2 else None
                aft_aft = aft
            elif aft_aft:
                rt.next = ListNode(aft_aft)
                break
            else:
                break
            
        return firstrt
    
if __name__ == "__main__":
    l1 = ListNode(2, next=ListNode(4, next=ListNode(3, next=None)))
    l2 = ListNode(5, next=ListNode(6, next=ListNode(4, next=None)))
    sol = Solution()
    sol.addTwoNumbers(l1, l2)