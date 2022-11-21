# findora cli

> Hight level user interface cli

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
| `eth` | BAR | `fra` | 1, 2, 3, 4, 6, 8 |
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




