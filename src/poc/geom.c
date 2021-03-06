#include <stdio.h>
#include <stdlib.h>
#include <locale.h>
#include <unistd.h>
#include <notcurses/notcurses.h>

// fun with the Geometric Shapes
int main(void){
  if(!setlocale(LC_ALL, "")){
    fprintf(stderr, "Couldn't set locale\n");
    return EXIT_FAILURE;
  }
  struct notcurses_options opts = {
    .flags = NCOPTION_INHIBIT_SETLOCALE | NCOPTION_NO_ALTERNATE_SCREEN,
  };
  struct notcurses* nc = notcurses_init(&opts, NULL);
  if(nc == NULL){
    return EXIT_FAILURE;
  }
  int dimy, dimx;
  struct ncplane* n = notcurses_stdplane(nc);
  ncplane_dim_yx(n, &dimy, &dimx);
  // leave a 20% total margin on the sides
  int margin = dimx / 5;
  const int xl = margin / 2;
  const int xr = dimx - xl;
  int flipmode = 0;
  struct timespec delay = { .tv_sec = 0, .tv_nsec = 250000000, };
  while(true){
    for(int i = xl ; i <= xr ; ++i){
      wchar_t w;
      if(ncplane_putwc_yx(n, 0, i, i % 2 == flipmode % 2 ? L'◪' : L'◩') <= 0){
        goto err;
      }
      w = L'🞯' + (flipmode % 6);
      if(ncplane_putwc_yx(n, 1, i, w) <= 0){
        goto err;
      }
      w = L'🞅' + (i + flipmode) % 5;
      if(ncplane_putwc_yx(n, 2, i, w) <= 0){
        goto err;
      }
      w = L'🞵' + (flipmode + 5) % 6;
      if(ncplane_putwc_yx(n, 3, i, w) <= 0){
        goto err;
      }
      switch(flipmode % 11){
        case 0: w = L'🞌'; break;
        case 1: w = L'🞍'; break;
        case 2: w = L'🞎'; break;
        case 3: w = L'🞏'; break;
        case 4: w = L'🞐'; break;
        case 5: w = L'🞑'; break;
        case 6: w = L'🞒'; break;
        case 7: w = L'🞓'; break;
        case 8: w = L'🞔'; break;
        case 9: w = L'🞕'; break;
        case 10: w = L'🞖'; break;
        default: goto err;
      }
      if(ncplane_putwc_yx(n, 4, i, w) <= 0){
        goto err;
      }
      w = ((i % 2) ? L'◴' : L'◰') + flipmode % 4;
      if(ncplane_putwc_yx(n, 5, i, w) <= 0){
        goto err;
      }
      if(ncplane_putwc_yx(n, 6, i, i % 2 == flipmode % 2 ? L'▱' : L'▰') <= 0){
        goto err;
      }
      w = L'▤' + flipmode % 4;
      if(ncplane_putwc_yx(n, 7, i, w) <= 0){
        goto err;
      }
      w = L'⌌' + flipmode % 4;
      if(ncplane_putwc_yx(n, 8, i, w) <= 0){
        goto err;
      }
      w = L'░' + flipmode % 3;
      if(ncplane_putwc_yx(n, 9, i, w) <= 0){
        goto err;
      }
      w = L'⯀' + flipmode % 5;
      if(ncplane_putwc_yx(n, 10, i, w) <= 0){
        goto err;
      }
    }
    if(notcurses_render(nc)){
      notcurses_stop(nc);
      return EXIT_FAILURE;
    }
    nanosleep(&delay, NULL);
    ++flipmode;
  }
  notcurses_stop(nc);
  return EXIT_SUCCESS;

err:
  notcurses_stop(nc);
  return EXIT_FAILURE;
}
