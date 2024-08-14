from string import ascii_letters, digits
class Solution:
    def isPalindrome(self, s: str) -> bool:
        list = [*filter(lambda c: c in ascii_letters + digits, s.lower())]
        print(list)
        if len(list) <= 1:
            return True
        if len(list) == 2:
            return list[0] == list[1]
        i,j = 0,len(list)-1
        while i < len(list)-1 and j > 0 and list[i] == list[j]:
            if i >= j:
                return True
            i += 1
            j -= 1
        return False
