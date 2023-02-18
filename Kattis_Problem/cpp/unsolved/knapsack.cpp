#include <iostream>
#include <utility>
#include <vector>
#include <bits/stdc++.h>


using namespace std;

int dp[2001][2001];
pair<int,int> VW[2001];

int main() {
    int C,n;
    while (cin >> C >> n) {
        for (int i=0;i < n;i++) {
            int v,w;
            cin >> v >> w;
            VW[i] = {v,w};
        }
        for (int v=1;v<=n;v++) {
            for (int w=1;w<=C;w++) {
                pair<int,int> pvpw = VW[v-1];
                int pv = pvpw.first;
                int pw = pvpw.second;
                dp[v][w] = dp[v-1][w];
                if (w >= pw && dp[v][w] < dp[v-1][w-pw]+pv) {
                    dp[v][w] = dp[v-1][w-pw]+pv;
                }
            }
        }
        vector<int> S(n);
        int i,res,w;
        i = n;
        res = dp[n][C];
        w = C;
        while (i > 0 && res > 0) {
            if (res == dp[i-1][w]) {
                continue;
            } else {
                S.push_back(i-1);
            }
            pair<int,int> vwi = VW[i-1];
            int v = vwi.first;
            int wi = vwi.second;
            res -= v;
            w -= wi;
            i--;
        }
        int x = S.size();
        cout << x << endl;
        cout << S[x-1];
        for (int i=x-1;i>=0;i--) {
            cout << S[i] << endl;
        }
        cout << endl;
    }
    return 0;
}
