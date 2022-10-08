UNAME := $(shell uname)

CC=gcc

ifeq ($(UNAME), Darwin)
	CFLAGS=-I/usr/local/opt/ncurses/include -L/usr/local/opt/ncurses/lib -lncurses -lmenu
endif

ifeq ($(UNAME), Linux)
	CFLAGS=-lncurses -lmenu
endif

l1t:
	mkdir -p ./build
	mkdir -p $(HOME)/.config/l1t/levels/
	cp ./levels/*.l1t $(HOME)/.config/l1t/levels/
	$(CC) $(CFLAGS) src/*.c -o ./build/l1t
dev:
	mkdir -p ./build
	mkdir -p $(HOME)/.config/l1t/levels/
	cp ./levels/*.l1t $(HOME)/.config/l1t/levels/
	$(CC) $(CFLAGS) -g3 -Wall -Wextra -fsanitize=address,undefined src/*.c -o ./build/l1t.dev
