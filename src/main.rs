use std::io::{Result, Write};

use select::document::Document;
use select::predicate::Class;
use stdinix::stdinix;
use ureq;

fn main() -> Result<()> {
    stdinix(|buf| {
        let body = ureq::get(&buf[..]).call().into_string()?;

        Document::from(&body[..])
            .find(Class("propertyCard-link"))
            .filter_map(|node| {
                node.attr("href").filter(|s| !s.trim().is_empty()).map(|s| {
                    println!("https://www.rightmove.co.uk{}", s);
                    std::io::stdout().flush()
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(())
    })
}
