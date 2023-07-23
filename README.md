Following along with https://os.phil-opp.com/


Original: https://github.com/phil-opp/blog_os

Fork: https://github.com/exokernel/blog_os

## Running the kernel with QEMU

Use `cargo run` which runs this command.

```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blogos/debug/bootimage-blogos.bin
```

This works because we included this in our `.cargo/config.toml`.

```toml
[target.'cfg(target_os = "none")']
runner = "bootimage runner"
```
