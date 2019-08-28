#include <curses.h>
#include <stdlib.h>
#include <unistd.h>
#include <errno.h>

#define PROGNAME "curses_init"

int main(void)
{
/* Verify terminal capabilities */
    if (!isatty(1) || initscr() == NULL) {
        fprintf(stderr, "%s: This program must be run from a terminal!\n",
                PROGNAME);
        return EXIT_FAILURE;
    }
    printf("%d", has_colors());

    attron(COLOR_PAIR(5) | A_BOLD );
    // mvprintw(LINES-1,0,"Press any key to end the program...");
    // refresh();
    // getch();
    endwin();

    return EXIT_SUCCESS;
}
