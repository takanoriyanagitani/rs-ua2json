use std::borrow::Cow;
use std::io;

use io::Write;

use woothee::parser::Parser;
use woothee::parser::WootheeResult;

#[derive(serde::Serialize)]
pub struct UserAgent<'a> {
    pub name: &'a str,
    pub category: &'a str,
    pub os: &'a str,
    pub os_version: Cow<'a, str>,
    pub browser_type: &'a str,
    pub version: &'a str,
    pub vendor: &'a str,
}

impl<'a> From<WootheeResult<'a>> for UserAgent<'a> {
    fn from(e: WootheeResult<'a>) -> Self {
        Self {
            name: e.name,
            category: e.category,
            os: e.os,
            os_version: e.os_version,
            browser_type: e.browser_type,
            version: e.version,
            vendor: e.vendor,
        }
    }
}

pub fn ua2parsed2json2writer<W>(p: &Parser, ua: &str, wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    let oparsed: Option<_> = p.parse(ua);
    let parsed: WootheeResult = oparsed
        .ok_or_else(|| format!("invalid user agent: {ua}"))
        .map_err(io::Error::other)?;
    let converted: UserAgent = parsed.into();
    serde_json::to_writer(wtr, &converted)?;
    Ok(())
}
