class Solution(object):
    def lengthOfLongestSubstring(self, s):
        """
        :type s: str
        :rtype: int
        """
        from collections import defaultdict
        word_dict = defaultdict(int)

        char_stack = []
        lastest_winner = []
        for char in s:
            if word_dict[char] >= 1: 
                if len(char_stack) > len(lastest_winner):
                    lastest_winner = char_stack[::]
                while char_stack[0] != char:
                    char_stack.pop(0)
                char_stack.pop(0) #remove the duplicate char
                char_stack.append(char)
                word_dict = defaultdict(int)  
                for c in char_stack:
                    word_dict[c] +=1   
                
            else: #firsttime
                char_stack.append(char)
                word_dict[char] += 1
        return len(lastest_winner) if len(lastest_winner) > len(char_stack) else len(char_stack)
    


if __name__ == "__main__":
    s = Solution()
    print(s.lengthOfLongestSubstring("abcabcbb")) #3
    print(s.lengthOfLongestSubstring("bbbbb")) #1
    print(s.lengthOfLongestSubstring("pwwkew")) #3
    print(s.lengthOfLongestSubstring(" ")) #1
    print(s.lengthOfLongestSubstring("dvdf")) #3
    print(s.lengthOfLongestSubstring("abba")) #2
    print(s.lengthOfLongestSubstring("aab")) #2
    print(s.lengthOfLongestSubstring("c")) #1
    print(s.lengthOfLongestSubstring("au")) #2


