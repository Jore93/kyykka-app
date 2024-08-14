# kyykka-app
Rust app for keeping Kyykkä scores

# Dev env
I used WSL2 to run my development environment. Within the folder I created a python virtual environment, where I installed cargo-lambda package with pip3.

To create environment run the following commands:
```
# Install python3
sudo apt install python3

# Install rust
sudo snap install rustup --classic
# Configure rust
rustup default stable

# Create python venv
python3 -m venv cargo-py
# Activate venv
source cargo-py/bin/activate

# Install cargo-lambda
pip3 install cargo-lambda
```

Now the environment is ready. A new lambda function can be initialized with command
```
cargo lambda new <function_name>
```

Before finishing close the virtual environment with command `deactivate`
