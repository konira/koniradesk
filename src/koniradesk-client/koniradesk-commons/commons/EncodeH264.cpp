//
// Created by lorib on 01/05/2024.
//

#include "EncodeH264.h"
#include <vector>
#include <iostream>
extern "C" {
     #include <libavcodec/avcodec.h>
     #include <libavformat/avformat.h>
}
namespace Encoder {

       std::vector<uint8_t> encodeRGBAtoH264(const std::vector<uint8_t>& rgba, int width, int height) {
        // Inicializar o codec H264
        const AVCodec* codec = avcodec_find_encoder(AV_CODEC_ID_H264);
        if (!codec) {
            std::cerr << "Codec H264 não encontrado.\n";
            return {};
        }

        AVCodecContext* context = avcodec_alloc_context3(codec);
        if (!context) {
            std::cerr << "Não foi possível alocar o contexto do codec.\n";
            return {};
        }

        context->width = width;
        context->height = height;
        context->pix_fmt = AV_PIX_FMT_RGBA;

        if (avcodec_open2(context, codec, nullptr) < 0) {
            std::cerr << "Não foi possível abrir o codec.\n";
            return {};
        }

        // Criar um frame AVFrame e preencher com os dados da imagem RGBA
        AVFrame* frame = av_frame_alloc();
        frame->format = context->pix_fmt;
        frame->width  = context->width;
        frame->height = context->height;

        int ret = av_frame_get_buffer(frame, 0);
        if (ret < 0) {
            std::cerr << "Não foi possível alocar os dados do frame.\n";
            return {};
        }

        // Copiar os dados da imagem RGBA para o frame
        memcpy(frame->data[0], rgba.data(), rgba.size());

        // Enviar o frame para o codec para codificação
        AVPacket* packet = av_packet_alloc();
        ret = avcodec_send_frame(context, frame);
        if (ret < 0) {
            std::cerr << "Erro ao enviar o frame para codificação.\n";
            return {};
        }

        // Receber o pacote codificado do codec
        ret = avcodec_receive_packet(context, packet);
        if (ret < 0) {
            std::cerr << "Erro ao receber o pacote codificado.\n";
            return {};
        }

        // Copiar os dados do pacote codificado para um vetor
        std::vector<uint8_t> encoded(packet->data, packet->data + packet->size);

        // Limpar
        av_packet_free(&packet);
        av_frame_free(&frame);
        avcodec_free_context(&context);

        return encoded;
    }

} // Encoder