sudo rust-lldb --batch --one-line "process attach --pid $(pidof tf_linux64)" \
  -o "expr void* dlopen(char*, int); void* handle = dlopen(\"$(realpath ./target/debug/liboxide.so)\", 2)" \
  -o "expr const char* dlerror(); const char* error = dlerror(); if (error) printf(\"Error: %s\n\", error)" \
  -o "settings set target.source-map . $(realpath .)" \
  -o "settings set target.source-map / $(realpath /)" \
  -o "continue"
