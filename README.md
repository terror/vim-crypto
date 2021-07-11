## vim-crypto

View live crypto currency information without leaving vim! (Note: Supports Neovim only)

### Demo

Here is a quick demo showcasing some of the basic commands:

[![asciicast](https://asciinema.org/a/1Rlx5v8VfFFVX5E0UPgy4BXqc.svg)](https://asciinema.org/a/1Rlx5v8VfFFVX5E0UPgy4BXqc)

### Installation

You can install this plugin using vim plug

```vim
Plug 'terror/vim-crypto', { 'do': 'cargo build --release' }
```

### Usage

The `vim-crypto` plugin currently only supports two commands:
- `CryptoTop` : View information regarding top crypto-currencies.
- `Crypto <SYMBOL>` : View information regarding crypto-currency with symbol
  `SYMBOL`.
