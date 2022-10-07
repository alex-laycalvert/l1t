l1t:
	mkdir -p ./build
	gcc src/*.c -lncurses -lmenu -o ./build/l1t
dev:
	mkdir -p ./build
	gcc -g3 -Wall -Wextra -fsanitize=address,undefined src/*.c -lncurses -lmenu -o ./build/l1t.dev
