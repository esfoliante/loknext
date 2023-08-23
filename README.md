# Loknext - Terminal Password Manager

[![GitHub license](https://img.shields.io/github/license/esfoliante/loknext)](https://github.com/esfoliante/loknext/blob/main/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/esfoliante/loknext)](https://github.com/esfoliante/loknext/issues)
[![GitHub stars](https://img.shields.io/github/stars/esfoliante/loknext)](https://github.com/esfoliante/loknext/stargazers)

## Introduction

Loknext is a secure and efficient terminal-based password manager, built using Rust. It provides a convenient way to manage your passwords and sensitive information right from your command line. In an age where digital security is of paramount importance, Loknext offers a robust solution for storing and accessing your credentials with ease.

### Why Loknext?

Password managers have become essential tools for modern-day cybersecurity. They help users generate strong, unique passwords for each of their accounts and store them securely. Loknext goes a step further by embracing the power of Rust, a systems programming language known for its performance, reliability, and safety.

With Loknext, you can:

- **Secure Your Passwords**: Loknext uses state-of-the-art encryption techniques to ensure your passwords are safe from prying eyes.

- **Command Line Convenience**: For those who prefer the terminal, Loknext is a command line password manager that streamlines password retrieval and storage.

- **Cross-Platform Compatibility**: Loknext is built in Rust, which means it can run on various platforms, making it accessible to all terminal-loving nerds.

- **Customization**: Loknext is highly configurable, allowing you to adapt it to your specific needs and preferences.

## Setup

To get started with Loknext, follow these simple steps:

1. **Clone the Repository**:

  ```sh
   git clone https://github.com/esfoliante/loknext.git
   cd loknext
   ```

2. **Copy the .env.example**

Copy the .env.example file and rename it to .env:

```sh
cp .env.example .env
```

3. **Edit the Environment File:**

Open the .env file and modify the ```CRYPT_TOKEN``` value to a strong, secret passphrase of your choice. This passphrase is crucial for encrypting and decrypting your stored passwords.

4. **Build Loknext**

Ensure you have Rust installed on your machine. If not, you can download it from Rust's official website.

Next, build Loknext using the following command:

```sh
cargo build
```

## Add to path

To make using Loknext even more convenient, you can add the Loknext binary to your computer's PATH environment variable. This allows you to run Loknext from any directory without specifying its full path.

Follow these steps to add Loknext to your PATH:

1. Find the location of the Loknext binary. It is typically located in the target/debug directory within the Loknext repository.

2. Add this directory to your PATH by appending the following line to your shell's configuration file (e.g., ~/.bashrc, ~/.zshrc, or ~/.profile):

```sh
alias loknext="cd /path/to/loknext/target/debug && ./loknext"
```

Replace /path/to/loknext/ with the actual path to the Loknext binary directory.

3. Save the configuration file and reload it by running:

```sh
source ~/.bashrc  # or source ~/.zshrc, depending on your shell
```

Now, you can run Loknext from anywhere in your terminal by simply typing ```loknext```.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contact

If you have any questions or need assistance, feel free to open an issue or contact me at ```miguel.personal@pm.me```.

Happy password managing with Loknext! 🛡️🔐
