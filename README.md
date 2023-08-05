## Convmits

Create conventional commits with an iteractive terminal, (soon the flags and commands will be available too if you prefer simple commands than interactive prompts)

## Installing the CLI

Convmits is currently developed on Rust language, and is tested against MacOS devices, although it may work in any device with the release builded version.

### Copy the Binary

Installation is as simple as cloning the repo and running the following command.

#### On Unix-based systems (Linux, macOS)
```sh
sudo cp target/release/convmits /usr/local/bin/
```

#### On Windows
```powershell
copy target\release\convmits.exe C:\path\to\directory\in\your\PATH\
```

You may be prompted to give `$PATH` permission for cloning the binary into your system.

### Create a Symbolic Link (symlink)

If you prefer not to add the binary into your system you can also create a symbolic link.

#### On Unix-based systems (Linux, macOS)
```sh
sudo ln -s "$(pwd)/target/release/convmits" /usr/local/bin/convmits
```

#### On Windows (using mklink)
```powershell
mklink C:\path\to\directory\in\your\PATH\convmits.exe "C:\path\to\your\rust\project\directory\target\release\convmits.exe"
```

## Using the command line tool

After copying or symlinking the binary, open a new terminal window or restart your shell. Now you should be able to use convmits as a command:

```bash
convmits
```

#### (WIP) Full use and docs of covnmits under construction