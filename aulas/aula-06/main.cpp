#include <iostream>

void troca(int v[], int i, int j) {
    int tmp = v[i];
    v[i] = v[j];
    v[j] = tmp;
}

void selection_sort(int v[], int n) {
    for (int i = 0; i < n - 1; i++) {
        int min_idx = i;
        
        for (int j = i + 1; j < n; j++) {
            if (v[j] < v[min_idx]) {
                min_idx = j;
            }
        }
        
        if (min_idx != i) {
            troca(v, i, min_idx);
        }
    }
}

int main() {
    int estoque[] = {64, 25, 12, 22, 11};
    int n = sizeof(estoque) / sizeof(estoque[0]);

    selection_sort(estoque, n);

    for(int i = 0; i < n; i++) {
        std::cout << estoque[i] << " ";
    }
    std::cout << std::endl;

    return 0;
}
