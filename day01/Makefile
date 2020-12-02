.PHONY: all clean watch

all:
	@rustc -g -O main.rs && ./main < input

clean:
	-rm -f main

watch:
	fswatch -o main.rs | xargs -I {} make
