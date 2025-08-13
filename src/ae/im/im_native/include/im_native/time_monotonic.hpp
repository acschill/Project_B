
#pragma once
#include <chrono>
namespace im_native {
inline unsigned long long mono_now_ns(){
    return std::chrono::duration_cast<std::chrono::nanoseconds>(
        std::chrono::steady_clock::now().time_since_epoch()).count();
}
}
