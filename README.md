# wayime

Inspired by [wlpinyin](https://github.com/xhebox/wlpinyin).

## Installation

### Nix

```bash
nix run github:xubaiwang/wayime
```

### Compile from source

```bash
cargo install --git https://github.com/xubaiwang/wayime
```

## Configuration

The configuration file is `$HOME/.config/wayime/config.toml`.

And the user rime data dir is `$HOME/.config/wayime/rime`.

```toml
# Use XF86_Keyboard in case conflict with other applications.
switch-key = "XF86_Keyboard"

# You can also use left Shift or any other Xkbsym name to switch
# switch-key = "Shift_L"

# for the name of keys, please refer to `examples/xkb_name.rs`.
```
