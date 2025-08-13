
#include "mm_native/crc32c.hpp"
namespace mm_native {
uint32_t crc32c(const void* data, size_t len) {
    const unsigned char* p = static_cast<const unsigned char*>(data);
    unsigned int h = 0; for (size_t i=0;i<len;++i) h = (h*131) ^ p[i]; return h;
}
}
