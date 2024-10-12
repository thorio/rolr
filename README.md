# rolr

![GitHub License](https://img.shields.io/github/license/thorio/rolr?style=flat-square)
![GitHub last commit](https://img.shields.io/github/last-commit/thorio/rolr?style=flat-square)
[![AUR Version](https://img.shields.io/aur/version/rolr-bin?style=flat-square)](https://aur.archlinux.org/packages/rolr-bin)

[![Demonstration](https://asciinema.org/a/xsIQnFL5oPSsyvB74qzsP0g56.svg)](https://asciinema.org/a/xsIQnFL5oPSsyvB74qzsP0g56)

rolr is a convenience tool to manage your local host with ansible. You define roles as ansible playbooks, then select the roles you want on a given system. Commit your config to a dotfiles repository and you have an automatic solution for setting up and keeping your various systems up to date.  
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
| Platform | x86-64 |  | aarch64 |
| --- | --- | --- | --- |
| Arch | [Package][arch-pkg] | [AUR][arch-aur] | | |
| Debian | [Package][debian-deb-x86] | | [Package][debian-deb-arm] |
| Linux | [Binaries][linux-tar-x86] | | [Binaries][linux-tar-arm] |

Note: The executable is fully statically linked and should therefore run basically anywhere, provided the architecture matches. Just unpack and go!

[arch-pkg]: https://github.com/thorio/rolr/releases/latest/download/rolr-x86_64.pkg.tar.zst
[arch-aur]: https://aur.archlinux.org/packages/rolr-bin
[debian-deb-x86]: https://github.com/thorio/rolr/releases/latest/download/rolr-x86_64.deb
[debian-deb-arm]: https://github.com/thorio/rolr/releases/latest/download/rolr-x86_64.deb
[linux-tar-x86]: https://github.com/thorio/rolr/releases/latest/download/rolr-x86_64.tar.gz
[linux-tar-arm]: https://github.com/thorio/rolr/releases/latest/download/rolr-aarch64.tar.gz


### Usage
Place ansible playbooks in `~/.config/rolr/roles`, separated by archicture and distro, e.g. `x86_64/ubuntu/10-base.yml`.  
rolr will only use playbooks from the matching arch+distro folder, so you can tailor playbooks to the systems they'll be running on.

Now run `rolr list` to view your roles, and `rolr add ROLE` to add them to the current system. Added roles can be re-run with `rolr update`.

Also see `rolr --help`.


### Tips

- Write idempotent playbooks to ensure updates work as expected. To quote [ansible's documentation][idempotency]:
  > An operation is idempotent if the result of performing it once is exactly the same as the result of performing it repeatedly without any intervening actions.

- Run `rolr select` to quickly view and add roles in a TUI menu.

- Run individual roles quickly with `rolr run ROLE`. This will only run that role, without adding to the list of active roles.

- Place an ansible config file in `~/.config/rolr/ansible.cfg` with these contents to quiet ansible down a bit:
  ```ini
  [defaults]
  display_skipped_hosts = false
  display_ok_hosts = false
  ```

- Playbooks will be run in alphanumerical order, use numbered prefixes to control this.
  If a role's name occurs multiple times, all instances will be run. Use this to split roles into multiple chunks and control the timing of their execution.

- A comment on the first line of the playbook will be interpreted as the description.

- Use symlinks for playbook re-use across distros and architectures.

- [Store your dotfiles in a bare git repo][dotfiles] to easily share and synchronize them across all your systems.  
  Do not commit `active.txt` and `playbook.yml`, as they are generated files.

[idempotency]: https://docs.ansible.com/ansible/latest/reference_appendices/glossary.html#term-Idempotency
[dotfiles]: https://harfangk.github.io/2016/09/18/manage-dotfiles-with-a-git-bare-repository.html


# Development

Using the devcontainer is highly encouraged to get up and running ASAP, otherwise:

- [Install Rust][rustup]
- Use [normal cargo commands][cargo] for development (`cargo build`, `cargo run`), use [`cargo-make`][cargo-make] for packaging.

[rustup]: https://www.rust-lang.org/tools/install
[cargo]: https://doc.rust-lang.org/cargo/
[cargo-make]: https://github.com/sagiegurari/cargo-make
