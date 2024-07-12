# rolr

![GitHub License](https://img.shields.io/github/license/thorio/rolr?style=flat-square)
![GitHub last commit](https://img.shields.io/github/last-commit/thorio/rolr?style=flat-square)

rolr is a convenience tool to manage your local host with ansible. You define roles as ansible playbooks, then select the roles you would like. Commit your config to a dotfiles repository and you have an automatic solution for setting up and keeping your various systems up to date.  
See this excellent [Youtube video][video] by Jeff Geerling to find out why you want this. Though the video primarily focuses on MacOS, the concept works on Linux just as well.

The author uses this in conjunction with a dotfiles repo and a [few bootstrapping scripts][bootstrapper] to manage various servers, desktops, laptops and WSL installations on multiple distros, keeping their configuration in sync.

This project is considered feature-complete.

[video]: https://www.youtube.com/watch?v=1VhPVu5EK5o
[bootstrapper]: https://github.com/thorio/thorio.github.io


## Features

- Simple, local architecture-as-code
- Powered by ansible
- Simple CLI for picking roles to install
- Write idempotent playbooks to keep your systems in sync


# Installation

Binaries are available for the following platforms:
| Platform | x86-64 | aarch64 |
| --- | --- | --- |
| Arch | [Package][arch-pkg] | |
| Debian | [Package][debian-deb-x86] | [Package][debian-deb-arm] |
| Linux | [Binaries][linux-tar-x86] | [Binaries][linux-tar-arm] |

Note: The executable is fully statically linked and should therefore run basically anywhere, provided the architecture matches. Just unpack and go!

[arch-pkg]: https://github.com/thorio/rolr/releases/latest/download/rolr-x86_64.pkg.tar.zst
[debian-deb-x86]: https://github.com/thorio/rolr/releases/latest/download/rolr-x86_64.deb
[debian-deb-arm]: https://github.com/thorio/rolr/releases/latest/download/rolr-x86_64.deb
[linux-tar-x86]: https://github.com/thorio/rolr/releases/latest/download/rolr-x86_64.tar.gz
[linux-tar-arm]: https://github.com/thorio/rolr/releases/latest/download/rolr-aarch64.tar.gz


### Usage
TODO

See `rolr --help`


# Development

Using the devcontainer is highly encouraged to get up and running ASAP, otherwise:

- [Install Rust][rustup]
- Use [normal cargo commands][cargo] for development (`cargo build`, `cargo run`), use [`cargo-make`][cargo-make] for packaging.

[rustup]: https://www.rust-lang.org/tools/install
[cargo]: https://doc.rust-lang.org/cargo/
[cargo-make]: https://github.com/sagiegurari/cargo-make
