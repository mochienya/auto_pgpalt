# auto_pgp
Small tool to to automatically get Abacus 2fa login code.

## how to use
1. copy PGP message from abacus's "Two-factor Authentication" page
2. run the program `autopgp-linux` or `autopgp-win` depending on your os
3. paste your new clipboard contents into the "Security code" field and proceed

## How can I trust this won't do something stupid?
Read the source code, `./src/main.rs` is less than 100 lines

All this does is:
1. Read clipboard content
2. Read your pgp key
3. Decode clipboard using pgp key
4. Use regex to find login key
5. Send that key to your clipboard