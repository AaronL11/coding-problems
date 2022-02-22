#include "heap.h"
#include "vector"
#include "cmath"
#include <algorithm>
#include <vector>

std::vector<int> heap;

int getMax(){
    return heap.size() ? heap[0] : -1;
}

int getSize(){
    return heap.size();
}

void insert(int element){
    heap.push_back(element);
    int n = heap.size();
    if (n==1) return;
    if (n==2) {
        if (heap[0] < heap[1]) {
            std::swap(heap[0],heap[1]);
        }
        return;
    }
    int position = n-1;
    int parent = (position-1)/2;
    while (position > 0 && heap[parent] < heap[position]) {
        std::swap(heap[parent],heap[position]);
        position = parent;
        parent = (position-1)/2;
    }
}

void removeMax() {
    int n = heap.size();
    if (n==0) return;
    if (n==1) {
        heap.pop_back();
        return;
    }
    heap[0] = heap[n-1];
    heap.pop_back();
    n = heap.size();
    int position = 0;
    int smallest = position;
    while (position*2 < n) {
        int left = position*2+1;
        int right = position*2+2;
        smallest = position;
        if (left < n && heap[left] > heap[smallest])
            smallest = left;
        if (right < n && heap[right] > heap[smallest])
            smallest = right;
        if (smallest != position) {
            std::swap(heap[position],heap[smallest]);
            position = smallest;
            continue;
        } else break;
    }
}
