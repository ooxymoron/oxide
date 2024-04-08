sudo rust-gdb -n -q \
  -ex "attach $(pidof hl2_linux)"\
  -ex "set \$dlopen = (void* (*)(char*, int))dlopen"\
  -ex "set \$dlerror = (char* (*)(void))dlerror"\
  -ex "call \$dlopen( \"$(realpath ./target/i686-unknown-linux-gnu/debug/liboxide.so)\", 2)"\
  -ex "call \$dlerror()"\
  -ex "layout src"\
  -ex "continue"\
