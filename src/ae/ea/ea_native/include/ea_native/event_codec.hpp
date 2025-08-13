
#pragma once
#include <vector>
#include <cstdint>
namespace ea_native {
std::vector<uint8_t> encode_varint(uint64_t v);
uint64_t decode_varint(const uint8_t* data, size_t len, size_t* used);
}
