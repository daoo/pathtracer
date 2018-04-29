FROM pritunl/archlinux:latest

RUN pacman -S --noconfirm pkgconfig meson ninja clang glew glm glfw freeimage

COPY . /pathtracer
WORKDIR /pathtracer

CMD ["sh", "-c", "meson builddir && ninja -C builddir test"]
