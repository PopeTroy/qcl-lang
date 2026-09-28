use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct QCLParser;

pub fn parse_qcl_source(src: &str) -> Result<()> {
    let pairs = QCLParser::parse(Rule::program, src)
        .map_err(|e| anyhow!("Parse error: {}", e))?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                println!("Successfully parsed QCL program tree.");
            }
            _ => {}
        }
    }
    Ok(())
}
