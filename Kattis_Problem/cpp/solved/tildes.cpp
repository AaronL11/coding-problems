#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;

#define FOR(i, a, b) for(int i = (a); i < (b); ++i)
#define REP(i, n) for(int i = 0; i < (n); ++i)
#define LSOne(S) ((S) & -(S))

int group[1000001];
int sz[1000001];

int findLead(int i) {
  while (group[i] != i) {
    group[i] = group[group[i]];
    i = group[i];
  }
  return i;
}

bool unionSet(int a, int b) {
  int a_lead = findLead(a);
  int b_lead = findLead(b);
  if (sz[b_lead] > sz[a_lead]) {
    swap(a_lead, b_lead);
  }
  if (a_lead == b_lead) {
    return false;
  }
  sz[a_lead] += sz[b_lead];
  group[b_lead] = a_lead;
  return true;
}

int main() {
  int n, q;
  scanf("%d%d", &n, &q);

  for (int i = 1; i < n+1; ++i) {
    group[i] = i;
    sz[i] = 1;
  }

  REP(i, q) {
    char instr;
    scanf("\n%c", &instr);
    if (instr == 't') {
      int a, b;
      scanf("%d%d", &a, &b);
      unionSet(a, b);
    } else {
      int a;
      scanf("%d", &a);
      printf("%d\n", sz[findLead(a)]);
    }
  }
  return 0;
}
