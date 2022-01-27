apt update -y && apt upgrade -y

apt install -y gcc-10 g++-10 build-essential ninja-build cmake clang
apt install -y python3-pip
apt install -y clang-13 lld-13 curl

curl https://sh.rustup.rs -sSf | sh -s -- -y
PATH="$HOME/.cargo/bin:$PATH"
rustup install nightly

pip install conan
