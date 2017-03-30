all:
	echo "Usage: make <target>"

%: src/%.cpp
	@mkdir -p bin
	g++ -std=c++11 -g -Wall -Wextra -o bin/$@ $^
	time ./bin/$@ < inputs/$@.in

clean:
	rm -rf bin/
