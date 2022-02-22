#include <bits/stdc++.h>
#include <utility>
#include <vector>

using namespace std;

#define ll long long
#define ld long double

int X[2000];
int Y[2000];
int R[2000];
bool checked[2000] = {false};

double dist(int i, int j) {
    double ans = sqrt(pow(X[i]-X[j],2)+pow(X[i]-X[j],2));
    return ans - R[i] - R[j];
}

int main() {
    int N;
    scanf("%d",N);
    for (int i=0; i < N; i++) {
        scanf("%d%d%d",&X[i],&Y[i],&R[i]);
    }
    checked[0] = true;
    priority_queue<pair<double,int>> queue;
    for (int i=1;i < N;i++){
        queue.emplace(-dist(0,i),i);
    }
    int n = 0;
    double total = 0;
    while (n < N-1) {
        pair<double, int> beam = queue.top();
        queue.pop();
        if (checked[beam.second]) {
            continue;
        }
        total += -beam.first;
        checked[beam.second] = true;
        ++n;
        for (int i=1; i < N; i++) {
            if (!checked[i]) {
                queue.emplace(-dist(beam.second,i),i);
            }
        }
    }
    printf("%.10f",total);
    return 0;
}


