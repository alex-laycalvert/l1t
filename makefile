l1t:
	mkdir -p ./build
	gcc src/*.c -lncurses -o ./l1t
dev:
	mkdir -p ./build
	gcc -g3 -Wall -Wextra -fsanitize=address,undefined src/*.c -lncurses -o ./build/l1t.dev
clean:
	rm -f ./build/l1t
	rm -f ./build/l1t.dev
	rm -f ./build/l1t.dev.dSYM
