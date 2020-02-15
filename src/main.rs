use std::io::{Result, Write};

use oops::Oops;
use select::document::Document;
use select::predicate::Class;
use stdinix::stdinix;
use ureq;

fn main() -> Result<()> {
    stdinix(|buf| {
        let body = ureq::get(&buf[..]).call().into_string()?;

        Document::from(&body[..])
            .find(Class("propertyCard-link"))
            .map(|node| {
                node.attr("href")
                    .filter(|s| !s.is_empty())
                    .map(|s| {
                        println!("https://www.rightmove.co.uk{}", s);
                        std::io::stdout().flush()
                    })
                    .oops(&format!("href missing for {:?}", buf))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(())
    })
}
