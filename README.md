# flexstrap

*Official bootstrapper, installer, and recipe builder for flexOS*

`flexstrap` is a bootstrapper utility for assembling the base system (RootFS) and deploying flexOS to disk. Written in Rust

---

## Key Features

* **Dual Operation Modes:**
  * **Offline Mode (`--repo <path>`):** Uses a local directory of pre-built `.fpk` packages without network access.
  * **Recipe Build Mode (Default):** Fetches build specifications from the flexOS packages repository, compiles packages on the fly, caches them, and extracts them into the target RootFS. (Work in progress)
* **Merged-usr FHS:** Automatically prepares a valid directory hierarchy (`/bin -> usr/bin`, `/lib -> usr/lib`, `/lib64 -> usr/lib64`).
* **fpm Database:** Generates the installed package state file `/var/lib/fpm/installed.json` from embedded package manifests.
* **Profile Support:** Selects system components during bootstrapping (`--profile base`, `--profile desktop`). (Currently working only base profile)

---

## Building

Requires the Rust compiler and Cargo package manager:

```bash
cargo build --release
```

The resulting executable will be located at target/release/flexstrap

## Usage

```bash
flexstrap <TARGET_DIR> [OPTIONS]
```

**Arguments and Flags:**
  * **<TARGET_DIR>** - Path to the directory where the system will be assembled (e.g., ./rootfs or /mnt)
  * **-p, --profile <NAME>** - Installation profile. Default: base
  * **-r, --repo <PATH>** - Path to a local directory containing pre-built .fpk archives

## Examples

1. **Assemble RootFS from local package cache:**
```bash
flexstrap ./rootfs --profile base --repo ./packages
```
2. **Standalone build from recipes:**
```bash
flexstrap /mnt/target --profile base
```
