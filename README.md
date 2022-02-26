# Crypto Wallet Generator

This is a utility to generate seed phrases and to generate crypto currency wallets from a seed phrase.
This way, you only need to remember one seed phrase and can generate wallets from it for multiple currencies.
A password can be added in the generation step so that you need both the seed phrase and the password to generate the wallets and access your funds.
We support both [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) and [scrypt](https://en.wikipedia.org/wiki/Scrypt) for generating the keys from the mnemonic (see details further below) and use [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)/[BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki) for derivation. Keys can be derived for Groestlcoin (GRS) wallets at the moment, further coins could be added relatively easily.

Generating a wallet from a seed phrase is a good way to secure your funds. You can, for example, print out the seed phrase (or etch it into metal cards for extra durability)
and store it offline. With this seed phrase (and the chosen password, if any), you can always restore access to your funds if the hard drive with your
crypto money happens to die. Or you carry it with you to get access to your funds from somewhere else.

## Usage

#### Generate seed phrase and a groestlcoin wallet for it

```
$ crypto-wallet-gen -c GRS
Password:
Repeat Password:
Mnemonic: curve office demand afford stomach rude hollow floor salmon daughter draft motor buffalo found float rail useful click other dismiss peace host scare moment
Password: [omitted]
Private Key: xprv9xtJUJ6oq5b7e6JU7eMfqx3giVJ82vbZiosXWu5Yaqfh9e3Rq3jLc6pwWJJG7dYd6Q3gpgEEuCY1ubC5S151hBwprfduVBeQrvHzhBxvKGk
```

The "mnemonic" is the seed phrase you need to remember or print.
The WIF can be entered to import the groestlcoin wallet in your favourite groestlcoin client.

Now say you loose access to your Groestlcoin wallet, using the phrase and step above, you can always recover your groestlcoin wallet:

```
$ crypto-wallet-gen -c GRS --from-mnemonic "curve office demand afford stomach rude hollow floor salmon daughter draft motor buffalo found float rail useful click other dismiss peace host scare moment"
Password:
Repeat Password:
Mnemonic: acid employ suggest menu desert pioneer hard salmon consider stuff margin over bus fiction direct useful tornado output forward wing cute chicken ladder hockey
Password: [omitted]
Private Key: xprv9xtJUJ6oq5b7e6JU7eMfqx3giVJ82vbZiosXWu5Yaqfh9e3Rq3jLc6pwWJJG7dYd6Q3gpgEEuCY1ubC5S151hBwprfduVBeQrvHzhBxvKGk
```

## Installation

#### 1. Install cargo (package manager for the rust programming language)

You can use [this one-step install command](https://www.rust-lang.org/tools/install).

You might have to call this afterwards, or alternatively just restart your bash session:
```
$ source $HOME/.cargo/env
```

Also make sure, you have openssl and a linker installed, for example by running the following:
```
$ sudo apt install libssl-dev pkg-config gcc
```

#### 2. Install crypto-wallet-gen
```
$ git clone https://github.com/Groestlcoin/crypto-wallet-gen && cd crypto-wallet-gen
$ cargo build
```

## How keys are derived

This tool uses [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) for the mnemonic and [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)/[BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki) derivation from your seed phrase and password with the derivation path `m/44'/{coin}'/{address}'`.
That is, for groestlcoin with address `17` (which is the default) we use `m/44'/17'/0'`.
For groestlcoin, the derived key can be directly used as a groestlcoin wallet. If such a key is imported into a groestlcoin client like electrum-grs, electrum-grs derives `m/{change}/{index}` from the key it is given, so the full derivation path will match the BIP44 scheme of `m/44'/{coin}'/{address}'/{change}/{index}`.

Examples generated at https://iancoleman.io/bip39/ .

### Scrypt derivation

There is an optional `--scrypt` parameter that replaces the [PBKDF2](https://en.wikipedia.org/wiki/PBKDF2) hash function of [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) with [scrypt](https://en.wikipedia.org/wiki/Scrypt).
This has three effects:

1. Somebody knowing your mnemonic but not the password who is trying to brute force the password will have a significantly harder time.
2. Generating a key from your mnemonic isn't instant anymore, it now takes several seconds (or minutes, depending on your hardware).
3. You're leaving BIP standards territory, there is no BIP standard for this. You cannot switch to a different tool and will be dependent on having this tool available when you want to generate keys from your mnemonic. Better keep a copy of the source code around just to be safe.
