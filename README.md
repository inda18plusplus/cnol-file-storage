# File Storage

This repository is split into two modules, a server and a client.

## Server
A HTTP server which processes requests made by the client. Currently all files are stored in memory.

## Client
Sends basic requests to the server, such as storing a new file or downloading a previously uploaded file.

## Cryptographic Algorithms

### File Hashing
Files are hashed using SHA256 digest since it is inexpensive to compute. 

### Signing and Encryption/Decryption 
Files are signed, encrypted and decrypted using an AEAD, the AES 128 GCM algorithm, using a key derived from a user supplied password using PBKDF2 with SHA256. 

PBKDF2 is used because SHA256 is computationally inexpensive. As such, it makes the encryption vulnerable to dictionary attacks. By iterating PBKDF2 a couple thousand times the cost of computing a key is increased, thus increasing the time it takes to brute-force a password. 

