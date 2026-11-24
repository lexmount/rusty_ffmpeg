# Bundled Linux H.264 sources

`ffmpeg-8.1.3-h264.tar.xz` contains the upstream FFmpeg build files and the
source trees for libavcodec, libavformat, libavutil, and libswscale. Licensing
notices are included in the archive. The `bundled_h264` feature builds these
libraries statically with only the H.264 decoder and parser and the MOV/MP4
demuxer enabled. The build has no FFmpeg network protocols or hardware
accelerators. Linux builds require a C toolchain, Make, tar, and xz. NASM is
optional on x86.

Recreate the archive from the official release.

```sh
curl -fLO https://ffmpeg.org/releases/ffmpeg-8.1.3.tar.xz
python3 vendor/pack_h264.py ffmpeg-8.1.3.tar.xz vendor/ffmpeg-8.1.3-h264.tar.xz
```
