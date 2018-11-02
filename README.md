# File Storage

This repository is split into two modules, a server and a client.

## Server
A HTTP server which processes requests made by the client. Currently all files are stored in memory.

### Valid URI Paths
| URI Path | HTTP Method | Description |
| --- | --- | --- |
| `/<file>` | `GET` | Responds with a file, with the id `<file>`, in raw binary |
| `/<file>` | `PUT` | Uploads a raw binary file, with the id `<file>`, and stores the file and it's hash in a Merkle tree. **Note:** The client is responsible for encrypting the file. |
| `/verify/root` | `GET` | Responds with the root hash (top hash) of the Merkle tree. |
| `/verify/<file>` | `GET` | Responds with the 16 256-bit hashes required to reconstruct the Merkle tree's root hash using the file with the ID `<file>`. The first hash is the sibling of the specified file's hash. The second hash is the sibling of the file's parent's hash, and so on. |

> File IDs are represented as a 16-bit unsigned integer and specify a file's location in the Merkle tree. When traversing the tree, starting at the root, the ID's least significant bit determines if the file is located to the left (0) or the right (1). One level down the tree the second bit determines the directon. Two levels down the third bit, and so on. This structure allows the client to reconstruct the root hash without explicitly knowing the location of all the hashes returned by `GET /verify/<file>`.

## Client
Sends basic requests to the server, such as storing a new file or downloading a previously uploaded file.

## Cryptographic Algorithms

### File Hashing
Files are hashed using SHA256 digest since it is inexpensive to compute. 

### Signing and Encryption/Decryption 
Files are signed, encrypted and decrypted using an AEAD, the AES 128 GCM algorithm, with a key derived from a user supplied password using PBKDF2 with SHA256. 

PBKDF2 is used because SHA256 is computationally inexpensive. As such, it makes the encryption vulnerable to dictionary attacks. By iterating PBKDF2 a couple thousand times the cost of computing a key is increased, thus increasing the time it takes to brute-force a password. 

