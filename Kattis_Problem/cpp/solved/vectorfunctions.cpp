#include "vectorfunctions.h"
#include <vector>
using namespace std;

// Reverse a vector.
// Note that it is sent as a reference, so you should
// reverse the same vector that was sent in.
void backwards(vector<int>& vec){
    int n = vec.size();
    for (int i=0;i < n/2;i++) {
        int temp = vec[i];
        vec[i] = vec[n-i-1];
        vec[n-i-1] = temp;
    }
}

// Return every other element of the vector, starting with the first.
// You should return a new vector with the answer.
// You are not allowed to modify the vector, even though it is
// sent as a reference. Therefore, the parameter is declared "const".
vector<int> everyOther(const vector<int>& vec){
    vector<int> w;
    int n = vec.size();
    for (int i=0;i < n;i++) {
        if (!(i%2)) {
            w.push_back(vec[i]);
        }
    }
    return w;
}

// Return the smallest value of a vector.
int smallest(const vector<int>& vec){
    int smallest = vec[0];
    int n = vec.size();
    for (int i=0;i < n;i++) {
        smallest = std::min(smallest,vec[i]);
    }
    return smallest;
}

// Return the sum of the elements in the vector.
int sum(const vector<int>& vec){
  int sum = 0;
  int n = vec.size();
  for (int i=0;i < n;i++) {
    sum += vec[i];
  }
  return sum;
}

// Return the number of odd integers, that are also on an
// odd index (with the first index being 0).
int veryOdd(const vector<int>& vec){
  int n = vec.size();
  int num = 0;
  for (int i=0;i < n;i++) {
    if (i%2 && vec[i]%2) {
      num += 1;
    }
  }
  return num;
}
