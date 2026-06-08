#include <cstdio>
#include <chrono>
#include <cstdint>

static inline double calculate(std::int64_t iterations, double param1, double param2) noexcept {
    double result = 1.0;
    double j1 = param1 - param2;  // starts at 3.0 for 4,1
    double j2 = param1 + param2;  // starts at 5.0 for 4,1
    const double step = param1;   // 4.0

    // Manual unrolling for better throughput while preserving exact operation order
    const std::int64_t UNROLL = 8;
    std::int64_t blocks = iterations / UNROLL;
    std::int64_t rem = iterations % UNROLL;

    while (blocks--) {
        result -= 1.0 / j1; result += 1.0 / j2; j1 += step; j2 += step;
        result -= 1.0 / j1; result += 1.0 / j2; j1 += step; j2 += step;
        result -= 1.0 / j1; result += 1.0 / j2; j1 += step; j2 += step;
        result -= 1.0 / j1; result += 1.0 / j2; j1 += step; j2 += step;
        result -= 1.0 / j1; result += 1.0 / j2; j1 += step; j2 += step;
        result -= 1.0 / j1; result += 1.0 / j2; j1 += step; j2 += step;
        result -= 1.0 / j1; result += 1.0 / j2; j1 += step; j2 += step;
        result -= 1.0 / j1; result += 1.0 / j2; j1 += step; j2 += step;
    }
    for (std::int64_t i = 0; i < rem; ++i) {
        result -= 1.0 / j1;
        result += 1.0 / j2;
        j1 += step;
        j2 += step;
    }
    return result;
}

int main() {
    using clock = std::chrono::high_resolution_clock;
    auto start_time = clock::now();

    double result = calculate(200000000, 4.0, 1.0) * 4.0;

    auto end_time = clock::now();
    std::chrono::duration<double> elapsed = end_time - start_time;

    std::printf("Result: %.12f\n", result);
    std::printf("Execution Time: %.6f seconds\n", elapsed.count());
    return 0;
}