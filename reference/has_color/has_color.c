#include <curses.h>
#include <stdlib.h>
#include <unistd.h>
#include <errno.h>
#include <assert.h>
#include <string.h>
 
#define PROGNAME "curses_init"
#define REQD_COLOR_PAIRS (5)
#define REQD_COLS (40)
#define REQD_LINES (7)
int main(void)
{
/* Verify terminal capabilities */
    if (!isatty(1) || initscr() == NULL) {
        fprintf(stderr, "%s: This program must be run from a terminal!\n",
                PROGNAME);
        return EXIT_FAILURE;
    }
    printf("%d", has_colors());
    exit(0);

    if (has_colors() && COLOR_PAIRS < REQD_COLOR_PAIRS) {
        printw("This program requires at least %d colors (%d found)\n", 
                REQD_COLOR_PAIRS, COLOR_PAIRS);
        goto cleanup_curses;
    }
    if (!has_colors()) {
        printw("This program requires colors (no support found)\n");
        goto cleanup_curses;
    }
    if (COLS < REQD_COLS || LINES < REQD_LINES) {
        printw("Terminal must be at least %dx%d!\n", REQD_COLS, REQD_LINES);
        goto cleanup_curses;
    }
     
/* Initialize all the colors */
    printw("This terminal supports %d colors\n", COLOR_PAIRS);
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    init_pair(3, COLOR_BLUE, COLOR_BLACK);
    init_pair(4, COLOR_CYAN, COLOR_BLACK);
    init_pair(5, COLOR_YELLOW, COLOR_BLUE);
    assert(5 <= REQD_COLOR_PAIRS);
     
    for (int i=1; i <= 5; i++) {
        attron(COLOR_PAIR(i));
        printw("This is color pair %d\n", i);
    }
    assert(6 <= REQD_LINES);
    assert(strlen("This is color pair XXX\n") <= REQD_COLS); 
    refresh();
 
/* End of program */
cleanup_curses:
    attron(COLOR_PAIR(5) | A_BOLD );
    mvprintw(LINES-1,0,"Press any key to end the program...");
    refresh();
    getch();
    endwin();
    return EXIT_SUCCESS;
}
