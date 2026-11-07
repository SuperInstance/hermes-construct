@echo off
"C:\Program Files\VideoLAN\VLC\vlc.exe" -I dummy --no-qt-privacy-ask --logfile=C:\Users\casey\vlc-run.log --verbose=1 dshow:// :dshow-vdev="ASUS FHD webcam" :dshow-adev=none :dshow-size=640x480 :live-caching=200 --sout "#transcode{vcodec=MJPG,vb=1500,fps=15}:std{access=http,mux=mpjpeg,dst=0.0.0.0:8090/proart.mjpg}" --sout-keep
