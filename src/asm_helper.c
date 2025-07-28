#include "asm_helper.h"

// ASM function declaration varies by platform
#ifdef _WIN32
extern long long multiply_and_add_ten(long long a, long long b);
#else
extern long long multiply_and_add_ten(long long a, long long b);
#endif

long long complex_math_operation(long long a, long long b) {
    return multiply_and_add_ten(a, b);
}
