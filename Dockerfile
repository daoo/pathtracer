FROM alpine:edge

RUN apk --no-cache add pkgconfig meson ninja gcc g++ clang glew-dev glm-dev freeimage-dev glfw-dev libxcursor-dev

COPY . /pathtracer
WORKDIR /pathtracer

CMD ["sh", "-c", "meson builddir && ninja -C builddir test"]
