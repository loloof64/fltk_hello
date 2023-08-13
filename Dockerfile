FROM appimagecrafters/appimage-builder:latest

RUN apt-get update --yes
RUN apt-get install --yes curl git libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN git clone https://github.com/loloof64/fltk_hello.git
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /fltk_hello
RUN cargo build --release
WORKDIR /

RUN mkdir -p /FltkHello.AppDir/usr/bin
RUN cp /fltk_hello/target/release/fltk_hello FltkHello.AppDir/usr/bin
RUN cp FltkHello.AppDir/usr/bin/fltk_hello FltkHello.AppDir/AppRun
RUN chmod +x FltkHello.AppDir/AppRun
RUN cp FltkHello.AppDir/usr/share/icons/hicolor/256x256/apps/icon.png FltkHello.AppDir/.DirIcon
RUN /usr/bin/appimagetool FltkHello.AppDir