//
// Created by lorib on 01/05/2024.
//

#ifndef ENCODEH264_H


#endif //ENCODEH264_H
// EncodeH264.h
#include <vector>
#include <cstdint>

namespace Encoder {
    std::vector<uint8_t> encodeRGBAtoH264(const std::vector<uint8_t>& rgba, int width, int height);
}

#define ENCODEH264_H