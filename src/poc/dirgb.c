#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <locale.h>
#include <notcurses.h>

// direct-mode generation of 4096 RGB foregrounds
int main(void){
  if(!setlocale(LC_ALL, "")){
    return EXIT_FAILURE;
  }
  struct ncdirect* nc = notcurses_directmode(NULL, stdout);
  if(!nc){
    return EXIT_FAILURE;
  }
  for(int r = 0 ; r < 256 ; r += 16){
    for(int g = 0 ; g < 256 ; g += 16){
      for(int b = 0 ; b < 256 ; b += 16){
        int ret = ncdirect_fg_rgb8(nc, r, g, b);
        if(ret){
          ncdirect_stop(nc);
          return EXIT_FAILURE;
        }
        printf("X");
        if(fflush(stdout) == EOF){
          fprintf(stderr, "Error flushing stdout (%s)\n", strerror(errno));
          return EXIT_FAILURE;
        }
      }
    }
  }
  printf("\n");
  if(ncdirect_stop(nc)){
    return EXIT_FAILURE;
  }
  return EXIT_SUCCESS;
}
