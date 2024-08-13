# battlenet-rs

Early days, but at POC for a Rust implementation of a Battlenet data wrapper.

## Goals

### MVP

- Implement and test all of World of Warcraft APIs
- Implement and test all of World of Warcraft Classic APIs
- Include example of rolling your own API call using this library
    - Do at least `wow_token_index` and something with args
    - Impetus is to show that the core client can be used for things that I'm
        probably not going to take the time to implement (Diablo, Hearthcraft,
        StarCraft II)
- Set up the repo on GitHub with a good contributors doc (so folks that are
    interested in the aforementioned games can do a PR)

### Stretch

- Implement caching to file and/or mongoDb
- Scanning for updates to static namespace (update cache)
    - Get associated index and compare to cached index
    - Pull instance for new IDs (remove cached for removed IDs?)
