sudo rust-gdb -n -q \
  -ex "attach $(pidof tf_linux64)"\
  -ex "set \$dlopen = (void* (*)(char*, int))dlopen"\
  -ex "set \$dlerror = (char* (*)(void))dlerror"\
  -ex "call \$dlopen( \"$(realpath ./target/debug/liboxide.so)\", 2)"\
  -ex "call \$dlerror()"\
  -ex "layout src"\
  -ex "continue"\
