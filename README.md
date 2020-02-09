Read from stdin, scrape rightmove search URLs, emit rightmove property URLS.

```bash
$ cargo run < <(xsv select 1 input.tsv)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/rightscrape`
https://www.rightmove.co.uk/property-to-rent/property-88835966.html
https://www.rightmove.co.uk/property-to-rent/property-88835966.html
...
```
