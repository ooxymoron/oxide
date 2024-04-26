
sudo gdb -n -q -ex "attach $(pidof hl2_linux)" -ex "continue" -ex "layout src"
