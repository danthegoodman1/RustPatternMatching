# basic_pattern_matcher

A basic pattern matcher for Rust using . as a delimiter, * for single level wildcard, ** for multi level wildcard.

This uses a trie-based architecture, and can probably be further optimized (see https://bravenewgeek.com/fast-topic-matching/ which finds that tries actually scale better)

Largely written by Gemini 2.5 Pro

## Concurrency

This does not support concurrency out of the box. For concurrent (thread safe) applications, some ideas to try would be:

- Using an actor thread, and handling small batches of requests from a channel (loop and grab what ever is available, up to N, then respond)
- Use Arc<Mutex> (simpler, potentially more overhead than actor model)

```
cargo add basic_pattern_matcher
```

M3 Max 128GB:

```
--- Adding Patterns ---
Added pattern: stock.nyse.*.price             ID: 101 Time: 18.834µs
Added pattern: stock.**.price                 ID: 101 Time: 1.125µs
Added pattern: stock.nasdaq.aapl.price        ID: 102 Time: 875ns
Added pattern: stock.*.ibm.price              ID: 103 Time: 834ns
Added pattern: stock.nyse.**                  ID: 104 Time: 5.291µs
Added pattern: stock.**                       ID: 105 Time: 375ns
Added pattern: finance.#                      ID: 106 Time: 500ns
Added pattern: *.nyse.ibm.*                   ID: 107 Time: 708ns
Added pattern: **.price                       ID: 108 Time: 709ns
Added pattern: stock.nyse.ibm.volume          ID: 109 Time: 792ns
--- Matching Topics ---
Topic: stock.nyse.ibm.price           Matches: [("stock.nyse.*.price", 101), ("stock.*.ibm.price", 103), ("stock.**.price", 101), ("stock.nyse.**", 104), ("*.nyse.ibm.*", 107), ("stock.**", 105), ("**.price", 108)]  Time: 13.917µs
Topic: stock.nasdaq.aapl.price        Matches: [("**.price", 108), ("stock.nasdaq.aapl.price", 102), ("stock.**.price", 101), ("stock.**", 105)]  Time: 1.417µs
Topic: stock.nyse.msft.price          Matches: [("**.price", 108), ("stock.nyse.*.price", 101), ("stock.**", 105), ("stock.**.price", 101), ("stock.nyse.**", 104)]  Time: 1.292µs
Topic: stock.nyse.ibm.volume          Matches: [("stock.**.price", 101), ("*.nyse.ibm.*", 107), ("stock.**", 105), ("stock.nyse.**", 104), ("stock.nyse.ibm.volume", 109), ("**.price", 108)]  Time: 4.458µs
Topic: stock.foo.bar.baz.qux          Matches: [("stock.**.price", 101), ("stock.**", 105), ("**.price", 108)]  Time: 1.083µs
Topic: finance.load                   Matches: [("**.price", 108)]  Time: 583ns
Topic: other.nyse.ibm.price           Matches: [("**.price", 108), ("*.nyse.ibm.*", 107)]  Time: 666ns
Topic: something.completely.different Matches: [("**.price", 108)]  Time: 542ns
Topic: stock.price                    Matches: [("**.price", 108), ("stock.**", 105), ("stock.**.price", 101)]  Time: 833ns
Topic: stock.nyse.goog.data           Matches: [("**.price", 108), ("stock.**", 105), ("stock.**.price", 101), ("stock.nyse.**", 104)]  Time: 2.458µs
```
