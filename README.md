# wikipedia_undumper
* Uses `quick_xml`
* You only need to provide a `BufReader` for the wikipedia undumper to work
* Serializes all possible fields according to the xml specification [from mediawiki](https://www.mediawiki.org/xml/export-0.10.xsd)
* Provides a stream of `Page` rust objects; no need to load the entire file into memory.

# usage

* with a decompressed xml wikipedia dump file

```rust
use std::{io::BufReader, fs::File};
use wikipedia_undumper::Undumper;

let filepath = "./path/to/wikipediadump.xml";
let f = File::open(filepath).unwrap();
let bufreader = BufReader::new(f);
let wikipedia_undumper = Undumper::from_reader(bufreader);
for page_result in wikipedia_undumper.into_iter() {
    // see src/schema.rs for what fields the page struct 
    let page = page_result.unwrap();
    println!("{:?}", page);
}
```

* For a bz2 compressed xml file you can use the rust [bz2 library](https://docs.rs/bzip2/latest/bzip2/)


```rust
use bzip2::read::MultiBzDecoder;
use std::{io::BufReader, fs::File};
use wikipedia_undumper::Undumper;

let filepath = "./path/to/wikipediadump.xml.bz2";
let f = File::open(filepath).unwrap();
let decompressor = MultiBzDecoder::new(f);
let bufreader = BufReader::new(decompressor);
let wikipedia_undumper = Undumper::from_reader(bufreader);
for page_result in wikipedia_undumper.into_iter() {
    // see src/schema.rs for what fields the page struct 
    let page = page_result.unwrap();
    println!("{:?}", page);
}
```
