CSV Data from https://raw.githubusercontent.com/tonmcg/County_Level_Election_Results_12-16/master/2016_US_County_Level_Presidential_Results.csv (via wget into the checkout directory).

After that, I ran:

```
python2 create_merkle_tree.py
```

That created directories and JSON documents for each voting district.

After that, I ran:

```
python2 update_merke_tree_hashes.py
```

That one just runs in a loop.  Sure that's not exactly friendly to the host machine, but this is only a proof of concept.

The Rust version is in the rust directory (ported by [Ash Levy](https://gitlab.com/ashkitten)). To run it, just `cd` to the directory and run:

```
cargo run -- ../data
```
