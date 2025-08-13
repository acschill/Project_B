
#pragma once
#include <chrono>
namespace ta_native {
inline uint64_t mono_now_ns(){
    return std::chrono::duration_cast<std::chrono::nanoseconds>(
        std::chrono::steady_clock::now().time_since_epoch()).count();
}
}
