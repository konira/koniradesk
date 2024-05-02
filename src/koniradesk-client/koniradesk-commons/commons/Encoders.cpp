// EncodeH264Wrapper.cpp
#include "EncodeH264.h" // Inclua o arquivo de cabeçalho
#include <vector>

extern "C" {
    void* encodeRGBAtoH264Wrapper(const uint8_t* rgba, int width, int height, int* output_size) {
        std::vector<uint8_t> rgba_vec(rgba, rgba + width * height * 4);
        std::vector<uint8_t> encoded = Encoder::encodeRGBAtoH264(rgba_vec, width, height);
        auto output = new uint8_t[encoded.size()];
        memcpy(output, encoded.data(), encoded.size());
        *output_size = static_cast<int>(encoded.size()); // Corrigir aviso de conversão
        return output;
    }
}