
#pragma once
#include <functional>
namespace mm_native {
class Watchdog{public: void on_crash(std::function<void()> cb){cb_=std::move(cb);} private: std::function<void()> cb_;};
}
