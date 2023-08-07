## Convmit

Create conventional commits with an iteractive terminal, (soon the flags and commands will be available too if you prefer simple commands than interactive prompts)

## Installing the CLI

Convmit is currently developed on Rust language, and is tested against MacOS devices, although it may work in any device with the release builded version.

### Copy the Binary

Installation is as simple as cloning the repo and running the following command.

#### On Unix-based systems (Linux, macOS)
```sh
sudo cp target/release/convmit /usr/local/bin/
```

#### On Windows
```powershell
copy target\release\convmit.exe C:\path\to\directory\in\your\PATH\
```

You may be prompted to give `$PATH` permission for cloning the binary into your system.

### Create a Symbolic Link (symlink)

If you prefer not to add the binary into your system you can also create a symbolic link.

#### On Unix-based systems (Linux, macOS)
```sh
sudo ln -s "$(pwd)/target/release/convmit" /usr/local/bin/convmit
```

#### On Windows (using mklink)
```powershell
mklink C:\path\to\directory\in\your\PATH\convmit.exe "C:\path\to\your\rust\project\directory\target\release\convmit.exe"
```

### Compile from source

For compiling from source you will need Rust installed and Cargo poackage manager installed too. After run this command:

```sh
cargo build --release
```

After building the binary follow the previous steps in `Copy the Binary` or `Create a Symbolic Link (symlink)`


## Using the command line tool

After copying or symlinking the binary, open a new terminal window or restart your shell. Now you should be able to use convmit as a command:

```bash
convmit
```

To enable the use of co authors in the commit message you will need to run the command: 

```bash
convmit init
```

#### (WIP) Full use and docs of covnmits under construction