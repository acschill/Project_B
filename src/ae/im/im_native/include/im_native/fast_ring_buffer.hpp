
#pragma once
#include <atomic>
#include <vector>
namespace im_native {
template<typename T>
class FastRingBuffer {
public:
    explicit FastRingBuffer(size_t cap):cap_(cap),buf_(cap),head_(0),tail_(0){}
    bool push(const T& v){auto h=head_.load(std::memory_order_relaxed);auto n=(h+1)%cap_;if(n==tail_.load(std::memory_order_acquire))return false;buf_[h]=v;head_.store(n,std::memory_order_release);return true;}
    bool pop(T& out){auto t=tail_.load(std::memory_order_relaxed);if(t==head_.load(std::memory_order_acquire))return false;out=buf_[t];tail_.store((t+1)%cap_,std::memory_order_release);return true;}
private:size_t cap_;std::vector<T> buf_;std::atomic<size_t> head_,tail_;
};
}
