use std::io::Result;

use select::document::Document;
use select::predicate::Class;
use stdinix::stdinix;
use ureq;


fn main() -> Result<()> {
    stdinix(|buf| {
        let body = ureq::get(&buf[..])
        .call()
        .into_string()?;

        Document::from(&body[..]).find(Class("propertyCard-link")).for_each(|node| {
            node.attr("href").filter(|s| s.len() > 0).map(|s| println!("https://www.rightmove.co.uk{}", s));
        });

        Ok(())
    })
}
