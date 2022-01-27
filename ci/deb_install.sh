apt update -y && apt upgrade -y

apt install -y build-essential ninja-build cmake git curl
apt install -y python3-pip
apt install -y gcc-10 g++-10 

curl https://sh.rustup.rs -sSf | sh -s -- -y
PATH="$HOME/.cargo/bin:$PATH"
rustup install nightly
cargo install -f cbindgen

pip install conan
