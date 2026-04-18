# Close Account Basics

This folder demonstrates how to securely create and close a Program Derived Address (PDA) data account using the Anchor framework in Solana.

## Overview
Closing an account reclaims its storage rent back in SOL to the payer and zeros out the data so it can't be reused maliciously. In this snippet, an account is created to store arbitrary data (a name), and an instruction is provided to close it securely by validating the `user` signer and utilizing the anchor `close` macro.

## Try it out
1. Navigate to the `anchor/` directory.
2. Run tests to execute the program securely:
   ```bash
   anchor test
   ```
   Alternatively, you can just run the typescript suite:
   ```bash
   yarn run ts-mocha -p tsconfig.json -t 1000000 'programs/anchor/tests/**/*.ts'
   ```
