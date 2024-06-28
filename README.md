Simple local password manager

### notes
- user data stored in users/
- no error checking in place
- typing in wrong password = program will crash on decryption attempt
- TODO: list command - list all service/password pairs

### usage
```
on linux: ./bin/linux/password-manager
on windows: .\bin\windows\password-manager.exe
```

### build it yourself
```
build linux: cargo build --target x86_64-unknown-linux-gnu
build windows: cargo build --target x86_64-pc-windows-gnu
```