#include <algorithm>
#include <bits/stdc++.h>
#include <ios>
#include <iostream>
#include <tuple>
#include <vector>
using namespace std;

#define ll long long
#define ld long double

#define rep(i, a, b) for(int i = a; i < (b); i++)
#define dbg(x) do { cerr << x << endl } while (0)
#define write(x) cout << x
#define writeln(x) cout << (x) << endl

tuple<ll,ll,ll> egcd(ll a, ll b) {
    if (b==0) {
        return make_tuple(a,1,0);
    } else {
        auto [g,x,y] = egcd(b,a%b);
        return make_tuple(g,y,x-(a/b)*y);
    }
}


int main() {
    ios_base::sync_with_stdio(0);
    cin.tie(NULL);
    ll t;
    cin >> t;
    while (t--) {
        ll K, a, n, b, m;
        cin >> a >> n;
        cin >> b >> m;
        K = n*m;
        auto [_, x, y] = egcd(n,m);
        ll r = (a*m*y + b*n*x) % K;
        if (r<0) { r = K + r; } else { r = r; };
        cout << r << " " << K << endl;
    }
    return 0;
}
