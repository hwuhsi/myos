# myos
my tiny os

thanks to [blog-os](https://os.phil-opp.com/) and [rcore](https://github.com/rcore-os/rCore)

to get started, first make sure you have rust installed(see [install rust](https://www.rust-lang.org/learn/get-started)) 

then run
```bash
cargo install bootimage
```

then

```bash
cargo run
```

(before run, you might need to add an img file in the testfs directory, or change the qemu command in Cargo.toml file)
