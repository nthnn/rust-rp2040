# Rust RP2040 Template

This Rust RP2040 project template aims to help the developers for a starting point on the development of embedded Rust with Raspberry Pi Pico. The template includes Cargo configurations, UF2 memory map, basic LED blinking program, and a simple Visual Studio Code configuration for [Rust Analyzer](https://code.visualstudio.com/docs/languages/rust) plugin.

## Getting Started

1. **Clone the project template**: Clone this Rust RP2040 project template.

    ```bash
    git clone --depth 1 https://github.com/nthnn/rust-rp2040
    ```

    This command clones the repository with a shallow copy (`--depth 1`), fetching only the latest commit history to save time and space.

2. **Install neccessary Rust components**: Before you can build and deploy your Rust program to RP2040-based boards, you need to install some Rust components:

    ```bash
    rustup target add thumbv6m-none-eabi
    cargo install elf2uf2-rs --locked
    cargo install cargo-binutils
    ```

3. **Build and flash your program** Once you have installed the necessary components, you can build and flash your Rust program to your RP2040-based board.

    **Build and Flash**

    To build and run your Rust program directly on the connected board:

    ```bash
    cargo run
    ```

    This command compiles your code and flashes the binary to the connected RP2040 board. Ensure your board is mounted and recognized by your system.

    **Build the Binary**

    If you prefer to build the binary without flashing immediately:

    ```bash
    cargo build --release
    ```

    This command compiles your code into a release build, generating the binary (`target/thumbv6m-none-eabi/release/<project_name>`).    

---

<p align="center">
    Happy Coding!<br/>
    <a href="https://github.com/nthnn">Nathanne Isip</a>
</p>