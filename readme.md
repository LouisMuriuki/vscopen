# Project Title

This is a simple command-line application written in Rust. It uses the `std::env` and `std::process` modules to interact with the system environment and processes.

## Getting Started

To run this application, you need to have Rust installed on your machine. You can download it from the [official Rust website](https://www.rust-lang.org/).

## Building and Installing

After cloning the repository, navigate to the project directory and build the application with the following command:

```bash
cargo build --release
This will create a binary in the target/release directory. You can move this binary to /usr/local/bin to make it globally accessible:

bash
Copy code
sudo mv target/release/open_vscode /usr/local/bin/open_vscode
Usage
Once installed, you can run this application from the command line with the following command:

bash
Copy code
open_vscode [arguments]
The application accepts any arguments and passes them to the code command. If the --help or -h argument is provided, the application will print "From source" to the console.

Error Handling
If there is an error while running the code command, the application will print the error message to the console and exit with a status code of 1.

Contributing
Contributions are welcome. Please feel free to submit a pull request.

License
This project is licensed under the MIT License.