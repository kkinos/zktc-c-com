# zktc-c-com

zktc-c-com is a ZKTC-C compiler implemented in Rust.

ZKTC-C is a C-like programming language for [ZKTC](https://github.com/kkinos/zktc).

# Install

```sh
cargo install --git https://github.com/kkinos/zktc-c-com.git
```

# Usage

Prepare zktc.c file. For example, prepare the following file.

`sample.zktc.c`

```c
int main()
{
	int x = 1;
	int y = 2;
	return x + y;
}
```

You can compile to a `asm` file.

```sh
zktc-c-com sample.zktc.c -o sample.asm
```

See `zktc-c-com -h` for other options.

# Tests

```bash
make test
```

requirements

- [zktc-asm](https://github.com/kkinos/zktc-asm)
- [zktc-emu](https://github.com/kkinos/zktc-emu)
- expect
