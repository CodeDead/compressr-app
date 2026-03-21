# Compressr-app

![Warning](https://img.shields.io/badge/warning-Under%20Development-yellow)
![GitHub](https://img.shields.io/badge/language-Rust-green)
![GitHub](https://img.shields.io/github/license/CodeDead/compressr-app)

![Compressr App](https://i.imgur.com/JlDa6Rf.png)

Compressr is the desktop application for compressing and optimizing images. It supports various image formats and provides a user-friendly interface for batch processing.

With Compressr, you can easily reduce the file size of your images without compromising on quality, making it ideal for web use, sharing, and storage.
The app also offers advanced features such as customizable compression settings and the ability to preserve metadata.

Whether you're a professional photographer or just looking to save space on your device, Compressr is the perfect tool for all your image optimization needs.

## Features

- [X] Batch processing: Compress multiple images at once.
- [X] Customizable compression settings: Adjust the level of compression to suit your needs.
- [X] User-friendly interface: Easy to navigate and use for all skill levels.
- [X] Support for various image formats: JPEG, PNG, GIF, and more.
- [X] Cross-platform compatibility: Available for Windows, macOS, and Linux.
- [X] Multi language support: Interface available in multiple languages.

## Building and Running the Application

To build the Compressr app, follow these steps:
1. Clone the repository:
   ```bash
   git clone https://github.com/CodeDead/compressr-app.git
   ```
2. Navigate to the project directory:
   ```bash
   cd compressr-app
   ```
3. Build the application using Cargo:
   ```bash
   cargo build --release
   ```
4. The compiled binary will be located in the `target/release` directory. You can run it directly from there or create a shortcut for easier access.
5. To run the application, use the following command:
   ```bash
   cargo run --release
   ```
   
### AppImage (Linux only)

To create an AppImage for Compressr, you can use the included Makefile. Run the following command in the project directory:
```bash
make release
```
This will generate an AppImage in the `target/release/AppImage` directory, which you can distribute and run on any compatible Linux system without needing to install it.

Optionally, you can pass a version argument to the Makefile to specify the version of the AppImage:
```bash
make release VERSION=1.0.0
```

## Dependencies

- [iced](https://github.com/iced-rs/iced)
- [image](https://crates.io/crates/image)
- [rfd](https://crates.io/crates/rfd)
- [iced_aw](https://crates.io/crates/iced_aw)
- [tokio](https://crates.io/crates/tokio)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)
- [log](https://crates.io/crates/log)
- [env_logger](https://crates.io/crates/env_logger)
- [reqwest](https://crates.io/crates/reqwest)
- [semver](https://crates.io/crates/semver)

## About

This library is maintained by CodeDead. You can find more about us using the following links:

* [Website](https://codedead.com)
* [Bluesky](https://bsky.app/profile/codedead.com)
* [Facebook](https://facebook.com/deadlinecodedead)

Copyright © 2026 CodeDead
