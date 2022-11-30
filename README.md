# findora cli

> Hight level user interface cli

## Guide

### Wallet

#### Create a root wallet

```shell
# Create root wallet
$ cli wallet --create --passphrase <pass> --lang <lang>

# Please backup output mnemonic
```

#### Show wallet

```shell
cli wallet --show

# This is output

FRA address: fra1xxxxx
FRA public key in hex: 0xXXXX
Amount: 0 FRA

ETH address: eth1xxxxx
ETH public key in hex: 0xXXXX
Amount: 0 FRA

EVM address: 0xXXXX
EVM public key in hex: 0xXXXX
# Totol FRA amount of this account, include BAR, EVM and ABAR.
Amount: 0 FRA

```

#### Generate account

```shell
cli wallet --generate --type <fra/eth/evm> --label <label>
```

#### Add account

```shell
cli wallet --add --private-key <private key> --label <label>
```

### Manage Asset

#### Show Asset

Show asset in brief

```shell
$ cli asset --show --address fra1XXXXXX

# Output:
Address: eth1XXXXXX
100 FRA
- 50 FRA(EVM,  0xXXXXXXXX)
- 50 FRA(BAR,  0x000000000)
- 0  FRA(ABAR, 0x000000000)

100 USDT
- 50 USDT(FRC20, 0xXXXXXXXX)
- 50 USDT(BAR,   0xXXXXXXXX)
- 0  USDT(ABAR,  0xXXXXXXXX)

100 BUSD
- 50 BUSD(BAR,   0xXXXXXXXX)
- 0  BUSD(ABAR,  0xXXXXXXXX)
```

#### Add Asset

```shell
# Add USDT on UTXO
$ cli asset --add --type utxo --asset 0xXXXXXXXX --symbol USDT

# Add USDT on EVM
$ cli asset --add --type evm-frc20 --asset 0xXXXXXXXX --symbol BUSD

# Add USDT based on auto
$ cli asset --add --type frc721 --asset 0xXXXXXXXX
```

## Functions

### Wallet

1. Generate mnemonic
2. Generate private key from mnemonic
3. Support `fra`, `eth`, `0x` address.

#### Address Type

- `fra` address is a ed25519 public key.
- `eth` address is a spec256k1 public key.
- `0x` address is a hashed address.

### Asset

Manage asset.

1. Add asset
2. Support sub-asset

#### Sub-asset

For example:

- FRA
   - BAR
   - ABAR
   - EVM

### Transfer

- From
- Sub-asset
- amount
- To


## Transaction Type

### Support Transaction type on findora

1. Non-Confidential BAR
2. Confidential amount BAR
3. Confidential asset type BAR
4. Confidential asset type and amount BAR
5. EVM call
6. BAR to EVM
7. EVM to AR
8. BAR to ABAR
9. ABAR to BAR
10. ABAR transfer

### Table of user operation

| From address type | From sub-asset | To address type | Support transaction type |
| - | - | - | - |
| `fra` | BAR | `fra` | 1, 2, 3, 4, 8 |
| `fra` | ABAR | `fra` | 9, 10 |
| `fra` | EVM | `fra` | |
| `fra` | BAR | `eth` | 1, 2, 3, 4, 6, 8 |
| `fra` | ABAR | `eth` | 9, 10 |
| `fra` | EVM | `eth` | |
| `fra` | BAR | `0x` | 6 |
| `fra` | ABAR | `0x` | |
| `fra` | EVM | `0x` | |
| `eth` | BAR | `fra` | 1, 2, 3, 4, 8 |
| `eth` | ABAR | `fra` | 9, 10 |
| `eth` | EVM | `fra` | 7 |
| `eth` | BAR | `eth` | 1, 2, 3, 4, 6, 8 |
| `eth` | ABAR | `eth` | 9, 10 |
| `eth` | EVM | `eth` | 5, 7 |
| `eth` | BAR | `0x` | 6 |
| `eth` | ABAR | `0x` | |
| `eth` | EVM | `0x` | 5 |
| `0x` | BAR | `fra` | |
| `0x` | ABAR | `fra` | |
| `0x` | EVM | `fra` | 7 |
| `0x` | BAR | `eth` | |
| `0x` | ABAR | `eth` | |
| `0x` | EVM | `eth` | 5, 7 |
| `0x` | BAR | `0x` | |
| `0x` | ABAR | `0x` | |
| `0x` | EVM | `0x` | 5 |

## Store

```shell
<home>
| - root-wallet.key
| - accounts
    | - fraxxxxx.key
    | - 0xxxxx.key
    | - ethxxx.key
| - assets
```

### Root wallet format

```rust
pub struct RootWallet {
   pub seed: String,
}
```

### Account Format

```rust
pub struct Account {
   pub private_key: String,
   pub ty: Type,
   pub label: String,
   pub address: String,
}
```



