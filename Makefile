.PHONY: $(MAKECMDGOALS)

REPO_DIR=${PWD}
BUILD_DIR=${REPO_DIR}/build
SOURCE_DIR=${REPO_DIR}/cxx

help:
	echo "Check Makefile"
test:
	cd rust_lib && cargo test

clean:
	rm -rf $(BUILD_DIR) my_rust_math/target
	mkdir -p ${BUILD_DIR}

prepare_rust_lib:
	cd my_rust_math && cargo build --release
	cbindgen my_rust_math --output=my_rust_math/include/my_rust_math.h --lang c++

prepare_gbench:
	bash ci/setup_gbench.sh

configure: prepare_rust_lib
	cmake -E make_directory ${BUILD_DIR}

	conan install ${SOURCE_DIR} --build=missing -if=${BUILD_DIR} -pr=ci/conan_gcc.txt

	cmake -S ${SOURCE_DIR} -B ${BUILD_DIR} \
		-DCMAKE_EXPORT_COMPILE_COMMANDS=1 \
		-DCMAKE_BUILD_TYPE=Release \
		-DCMAKE_C_COMPILER=gcc-11 \
		-DCMAKE_CXX_COMPILER=g++-11 \
		-G Ninja

	ln -sf ${BUILD_DIR}/compile_commands.json ${SOURCE_DIR}


build: 
	cmake --build ${BUILD_DIR} -j

run: 
	${BUILD_DIR}/bin/main

bench:
	${BUILD_DIR}/bin/bench