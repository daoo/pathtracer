FROM ubuntu:rolling
RUN apt-get update && apt-get -y install build-essential meson ninja-build clang libglew-dev libglm-dev libglfw3-dev libfreeimage-dev pkg-config
ADD . /root

# Using alpine instead (not working due to glm packaging problems)
# FROM alpine:edge
# RUN echo http://nl.alpinelinux.org/alpine/edge/testing >> /etc/apk/repositories
# RUN apk update && apk add --no-cache build-base pkgconf meson ninja clang glew-dev glm-dev glfw-dev freeimage-dev libxcursor-dev
# ADD . /root
