# Varlon-cli

Welcome to the Varlon-cli repository! This project provides a templates for creating VARA projects with ease.

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Build Instructions

1. **Clone the Repository**

   ```bash
   git clone https://github.com/whispercash/vara-contract-generator.git
   cd vara-contract-generator
   ```

2. **Build the Project**

   To build the project, run the following command:

   ```bash
   cargo build -- --name test_project --description "a vara project template"
   ```

3. **Locate the Contract**

   After building the project, navigate to the following directory to find the contract:

   ```bash
   cd target/wasm32-unknown-unknown/release/
   ```

   Your contract will be present in the `test_project.opt.wasm` file.

### Release Build

For a release build, use the following command:

```bash
cargo build --release
```

To run the release build, use:

```bash
target/release/vara --name test_project
```

## Contributing

We welcome contributions! Please feel free to submit issues and pull requests.

### Steps to Contribute

1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Make your changes
4. Commit your changes (`git commit -am 'Add new feature'`)
5. Push to the branch (`git push origin feature-branch`)
6. Create a new Pull Request

## Contact

For any queries or issues, please reach out to us at:

- Email: [varlonclisupport@gmail.com](varlonclisupport@gmail.com)
- Twitter: [@Whispercash](https://twitter.com/0xwhispercash)



We hope this template helps you get started with your VARA projects smoothly. Happy coding! 🚀
