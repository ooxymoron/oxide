
sudo gdb -n -q -ex "attach $(pidof tf_linux64)" -ex "continue" -ex "layout src"
