# Encoding/Decoding Programming Challenge
Library with sample encoding and decoding functions based on ALG-C (LZ77). 

## To use
Set nightly: (required by datatest crate):

```rustup override set nightly```

Build and run binary with custom input:

```cargo run <input>```

Run library correctness and efficiency tests: 

```cargo test --test integration -- --color always --nocapture```

You can modify the search buffer and lookahead windows in the global variables at the top of `tests/lib.rs`. Runtime on the large test data is reasonably slow. Consider the performance on the E. Coli genome, which is 4.6 MB: (`tests/data/large/E.coli`)

| search buff | lookahead | encoding time | compression ratio |
| ----------- | --------- | ------------- | ----------------- |
| 6           | 4         | 00:04.501699  | 0.74097025        |
| 10          | 7         | 00:10.935680  | 0.86347365        |
| 15          | 10        | 00:15.871778  | 0.9657823         |
| 20          | 10        | 00:39.840204  | 1.0345451         |
| 50          | 30        | 03:40.915504  | 1.2718489         |
| 100         | 60        | 09:38.043889  | 1.4508238         |






