use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer, from_reader, ser::PrettyFormatter};
use ureq::Agent;

use crate::models::{ScoopEntry, ZigVersionIndex};

mod models;

#[macro_use]
mod macros;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./bucket/zig-nightly.json")?;

    let agent = Agent::config_builder().timeout_global(Some(Duration::from_mins(30))).build().new_agent();
    let resp = agent.get("https://ziglang.org/download/index.json").call()?.body_mut().read_to_string()?;

    let idx = ZigVersionIndex::deserialize(&mut Deserializer::from_str(&resp))?;

    println!("{}", idx.master());

    let mut zig = from_reader::<_, ScoopEntry>(BufReader::new(file))?;

    if zig.version != idx.master().version {
        let mut ser = Serializer::with_formatter(BufWriter::new(File::create("./bucket/zig-nightly.json")?), PrettyFormatter::with_indent(b"    "));

        zig.version = String::from(idx.master().version);
        zig.architecture.update(idx.master());

        zig.serialize(&mut ser)?;
    }

    Ok(())
}
