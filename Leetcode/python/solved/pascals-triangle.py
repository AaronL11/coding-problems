class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        tri = [[1]]
        if numRows == 1:
            return tri
        n = 1
        while n < numRows:
            row = [1]
            for i in range(1,len(tri[n-1])):
                row.append(tri[n-1][i]+tri[n-1][i-1])
            row.append(1)
            tri.append(row)
            n += 1
        return tri
