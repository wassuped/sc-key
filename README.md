ScKey

Rust port of [ScKey](https://github.com/FMZNkdv/ScKey)

Quick Start

Method 1: Direct key deobfuscation

```bash
cargo run <obfuscated key>
```

Method 2: Extract and deobfuscate from libg

```bash
cargo run libg.so
```

Using pre-built binaries

1. Download the latest release from the [Releases page](https://github.com/wassuped/sc-key/releases/latest)
2. Rename file to "sc-key"
3. Run:
   ```bash
   ./sc-key <obfuscated key or libg>
   ```

Building from source

```bash
git clone https://github.com/wassuped/sc-key
cd sc-key
cargo run 
```

---

This content is not affiliated, approved, sponsored or endorsed by Supercell and Supercell is not responsible for it. For more, see the Supercell Fan Content Policy: www.supercell.com/fan-content-policy
