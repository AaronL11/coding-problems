# include <bits/stdc++.h>
using namespace std;

unordered_map<vector<int>,int> memo;
int grid[100];

int cover(int n,int w,int h) {
    if (n == 0) {
        return 0;
    } else {
        bool acc = true;
        for (int i=0;i < w-1;i++) {
            acc &= grid[i]==grid[i+1];
        }
        int sum = (acc) ? 0 : 1;
        for (int i=0; i<w; i++) {
            if (grid[i]+1 <= h) {
                grid[i] += 1;
                if (memo.count(grid) != 0) {
                    sum += memo[grid];
                }
                grid[i] -= 1;
            }
        }
        memo.insert({ grid, sum });
        return sum;
    }
}

int main() {
    int n,w,h;
    scanf("%s %s %s",&n,&w,&h);
    
    return 0;
}
