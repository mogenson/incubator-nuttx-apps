#include <nuttx/config.h>
#include <stdio.h>

// Rust entry point: defined in src/lib.rs
int rust_main(int argc, char *argv[]);

// C entry point: should be named PROGNAME_main
int main(int argc, char *argv[]) {
  int ret = rust_main(argc, argv);

  printf("Done!\n");
  return ret;
}
