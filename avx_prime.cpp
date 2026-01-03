#include <immintrin.h>

void matrix_mul_avx512(const float* A, const float* B, float* C, int N) {
    for (int i = 0; i < N; ++i) {
        for (int k = 0; k < N; ++k) {
            __m512 va = _mm512_set1_ps(A[i * N + k]);
            for (int j = 0; j < N; j += 16) {
                __m512 vb = _mm512_loadu_ps(&B[k * N + j]);
                __m512 vc = _mm512_loadu_ps(&C[i * N + j]);
                _mm512_storeu_ps(&C[i * N + j], _mm512_fmadd_ps(va, vb, vc));
            }
        }
    }
}
