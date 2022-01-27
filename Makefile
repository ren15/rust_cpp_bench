.PHONY: $(MAKECMDGOALS)

BUILD_DIR=build

COMPILE_CMD=clang++ cxx/main.cpp -o ${BUILD_DIR}/main \
		-O3 -std=c++20 \
		-lmy_rust_math -L my_rust_math/target/release \
		-I my_rust_math/include


help:
	echo "Check Makefile"
test:
	cd rust_lib && cargo test
gen_compile_commands:
	bear -- ${COMPILE_CMD}
	mv compile_commands.json cxx/compile_commands.json


clean:
	rm -rf $(BUILD_DIR) my_rust_math/target
	mkdir -p ${BUILD_DIR}

gen_rust_so: clean
	cd my_rust_math && cargo build --release

cbindgen:
	cbindgen my_rust_math --output=my_rust_math/include/my_rust_math.h --lang c++

build: gen_rust_so cbindgen
	${COMPILE_CMD}

run: build
	LD_LIBRARY_PATH=./my_rust_math/target/release ${BUILD_DIR}/main