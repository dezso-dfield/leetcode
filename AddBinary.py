class Solution:
    def addBinary(self, a: str, b: str) -> str:
        result:str = ""
        i:int = len(a)-1
        j:int = len(b)-1
        carry:int = 0
        while i >= 0 or j >= 0 or carry:
            current_sum:int = carry
            if i >= 0:
                current_sum += int(a[i])
                i -= 1
            if j >= 0:
                current_sum += int(b[j])
                j -= 1
            result += str(current_sum % 2)
            carry = current_sum //  2
        return result[::-1]
