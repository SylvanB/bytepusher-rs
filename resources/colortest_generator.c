#include <stdio.h>
#include <string.h>

/* <<< Colortest for the BytePusher VM             >>>
 * <<< By Javamannen (Adam Danielsson)  2010-09-26 >>>
 *
 * This program generates "Palette Test.BytePusher",
 * which displays the BytePusher's fixed 256-color palette.
 * Colors 00..D7 hex form a 6*6*6 colorcube (index=red*36+green*6+blue).
 * Colors D8..FF hex are set to black (0,0,0).
 *
 * To run the generated program you need a BytePusher VM,
 * which can be downloaded from the BytePusher page at:
 * http://esolangs.org/wiki/BytePusher.
 * If you've created a BytePusher program or VM, please post it on that page!
 */

enum {PAGE = 256, BANK = PAGE*PAGE, MEMSIZE = PAGE*BANK};
unsigned char mem[MEMSIZE];
int pc;
enum {ADDR = 3, INSTR = 3*ADDR};
#define next (pc+INSTR)
#define prev (pc-INSTR)

// zeropage
enum {
  KEYS  = 0x00,
  RESET = 0x02,
  GFX   = 0x05,
  SND   = 0x06
};
// pages
int samples, prog, end;
// banks
int pixels;

void org (int a) {pc  = a;}
void skip(int x) {pc += x;}
void next_page(void) {pc  = (pc & ~(PAGE-1)) + PAGE;}
void next_bank(void) {pc  = (pc & ~(BANK-1)) + BANK;}
void byte(int a) {mem[pc++] = a;}
void word(int a) {byte(a >> 8) ; byte(a);}
void addr(int a) {byte(a >> 16); word(a);}

// Mnemonics
void bbj (int a, int b, int c) {addr(a); addr(b); addr(c);}
void mov (int a, int b) {bbj(a, b, next);}
void jmp (int c       ) {bbj(0, 0, c   );}
void nop (void        ) {bbj(0, 0, next);}
void wait(void        ) {bbj(0, 0, pc  );}

int color(int r, int g, int b) {return r*36 + g*6 + b;};

void assemble(void) {
  org(prog);
  wait();
}

void save_file(void) {
  FILE *f = fopen("Palette Test.BytePusher", "wb");
  fwrite(mem, 1, end, f); fclose(f);
}

void init_zeropage(void) {
  org (RESET); addr(prog);
  byte(pixels  / BANK);
  word(samples / PAGE);
}

void init_memmap(void) {
  org(PAGE);
  samples = pc; next_page();
  prog    = pc; next_bank();
  pixels  = pc; next_bank();
  end     = pc;
}

void print_hexdigit(int value, int x, int y) {
  static unsigned char hexdigits[4*16*5] =
  ".X...X..XX..XX..X.X.XXX..XX.XXX.XXX.XXX..X..XX...XX.XX..XXX.XXX."
  "X.X.XX....X...X.X.X.X...X.....X.X.X.X.X.X.X.X.X.X...X.X.X...X..."
  "X.X..X...XX..X..XXX.XXX.XXX..X...X..XXX.XXX.XX..X...X.X.XX..XXX."
  "X.X..X..X.....X...X...X.X.X..X..X.X...X.X.X.X.X.X...X.X.X...X..."
  ".X...X..XXX.XX....X.XX..XX...X..XXX..X..X.X.XX...XX.XX..XXX.X...";
  int xx, yy, c = color(5,5,5);
  for(yy = 0; yy < 5; yy++) {
    for (xx = 0; xx < 3; xx++) {
      if (hexdigits[yy*64+value*4+xx] == 'X')
        mem[pixels+(y+yy)*256+x+xx] = c;
    }
  }
}

// Print an 8-bit hex number
void print_hex(int value, int x, int y) {
  print_hexdigit(value>>4, x  , y);
  print_hexdigit(value&15, x+4, y);
}

// Show colors 00..D7 hex
void make_a_pretty_little_picture(void) {
  int r,g,b,x,y,xx,yy,c;
  for (r=c=0; r < 6; r++) {
    for (g = 0; g < 6; g++) {
      yy = r*36+g*6+1;
      for (b = 0; b < 6; b++,c++) {
        xx = b*42+1;
        print_hex(c, xx, yy); xx += 8;
        for (y = 0; y < 5; y++) {
          for (x = 0; x < 33; x++) {
            mem[pixels+(yy+y)*256+xx+x] = c;
          }
        }
      }
    }
  }
}

// Show colors D8..FF hex
void show_unused_colors(void) {
  int x,y,xx,yy,x2,y2,c=216;
  for (y = 0; y < 4; y++) {
    yy = y*6+217;
    for (x = 0; x < 12; x++,c++) {
      xx = x*21+1;
      if (c < 256) {
        print_hex(c, xx, yy); xx+=8;
        for (y2 = 0; y2 < 5; y2++)
          for (x2 = 0; x2 < 12; x2++)
            mem[pixels+(yy+y2)*256+(xx+x2)] = c;
      }
    }
  }
}

int main(int argc, char *argv[]) {
  init_memmap();
  init_zeropage();
  memset(mem+pixels, color(2,2,2), 65536); // gray background
  make_a_pretty_little_picture();
  show_unused_colors();
  assemble();
  save_file();
  return 0;
}
