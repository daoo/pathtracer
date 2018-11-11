FROM alpine:edge

# As of 2018-11-11 freeimage was only available in testing
RUN echo "http://dl-cdn.alpinelinux.org/alpine/edge/testing" >> /etc/apk/repositories
RUN apk --no-cache add pkgconfig meson ninja gcc g++ clang glew-dev glm-dev freeimage-dev glfw-dev libxcursor-dev

COPY . /pathtracer
WORKDIR /pathtracer

CMD ["sh", "-c", "meson builddir && ninja -C builddir test"]
