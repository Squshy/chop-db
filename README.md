<h2 align="center">
    Chop DB
</h2>

<p align="center">
    A log structured key-value store.
</p>

# The database

The `db` folder holds the logic for the database. It is a simple log-structured database storing key-value pairs. The data is stored as bytes in the file, along with how large the data is. Storing this allows us to quickly look up data using hash-maps which point to where the data starts in the file.

# Using the database 

The database library can be imported into Rust projects and used freely. This may not always be desirable so a simple CLI/server is provided. The client and the server communicate via gRPC. The client is currently written in Go just for fun to showcase how gRPC can be used across different languages. The CLI accepts the `get`, `set`, and `delete` commands the database supports.

# What's left?

A lot. This project is a part of the engineering book club I started at PostGrid and is ever expanding as we go through [Designing Data Intensive Applications](#https://dataintensive.net/).

Some of the future tasks include:
    - [ ] Allow for persistent start-ups by reading existing log files
    - [ ] Compact log files in the background 
    - [ ] Add support for leader/follower database replication 
