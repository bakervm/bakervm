use super::parser::Rule;
use core::error::*;
use core::typedef::*;
use pest::iterators::Pairs;

pub struct BeastCompiler;

impl BeastCompiler {
    pub fn compile(pairs: Pairs<Rule>) -> Result<ImageData> {
        for pair in pairs {
            let pair = pair.into_inner()
                .next()
                .chain_err(|| "unable to fetch rule")?;

            match pair.as_rule() {
                Rule::module => return Ok(ImageData::new()),
                _ => bail!("no module found"),
            }
        }

        Ok(ImageData::new())
    }
}
