class Solution:
    def reverseBits(self, n: int) -> int:
        return int(f"0b{''.join(reversed(bin(n)[2:])):032}", base=0)
        
