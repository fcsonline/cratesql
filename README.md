# cratesql

A GraphQL interface for crates.io repository.

This repository is in active development. Do not expect much stability.

All exposed information by this service is retrieved by the public dump ([dump](https://static.crates.io/db-dump.tar.gz))
specified [here](https://crates.io/data-access)

You can interact with this API starting a local server or going to this public
server. ~~Here~~

# Setup

crates.io platform uses a Postgresql database.

```
./setup.sh
```

# Development

```
diesel print-schema
```

# Roadmap

This is the current list of all tables present in database dump. Checked items
represent those that are exposed.

- [X] Crates
  - [] Crate categories
  - [] Crate keywords
  - [] Crate dependencies
- [] Categories
- [] Keywords
- [] Dependencies
- [] Badges
- [] Crate owner invitations
- [] Crate owners
- [] Teams
- [] Users
- [] Versions
  - [] Version downloads
  - [] Version owner actions
  - [] Version published by

- [] Filter capabilities for all main entities
- [] Pagination capabilities for all main entities


- [] Metadata
- [] Publish Rate Override
- [] Readme renderings
- [] Reserved crate names
- [] Follows
- [] Emails
- [] Api Tokens
- [] Background Jobs
