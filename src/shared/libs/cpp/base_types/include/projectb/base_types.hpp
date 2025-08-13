#pragma once
#include <cstdint>
#include <vector>

namespace projectb {

enum class Mode : uint8_t { Idle=0, Observe=1, Focus=2, React=3 };

struct Envelope {
    std::uint64_t seq_no{};
    std::uint64_t ts_mono_ns{};
    std::uint32_t schema_ver{};
    std::uint32_t source_id{};
    std::vector<std::uint8_t> payload{};
    std::uint32_t checksum{};
};

} // namespace projectb