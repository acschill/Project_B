
#include "im_native/event_codec.hpp"
namespace im_native {
std::vector<uint8_t> encode_varint(uint64_t v){std::vector<uint8_t> o;while(v>=0x80){o.push_back((uint8_t)(v|0x80));v>>=7;}o.push_back((uint8_t)v);return o;}
uint64_t decode_varint(const uint8_t* d,size_t len,size_t* used){uint64_t v=0;int s=0;*used=0;for(size_t i=0;i<len;++i){uint8_t b=d[i];v|=(uint64_t)(b&0x7F)<<s;*used+=1;if(!(b&0x80))break;s+=7;}return v;}
