use std::error::Error;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Server to send post request
    #[arg(short, long)]
    pub server: String,

    /// Data to post(deprecate)
    #[arg(short, long,default_value="Hello, World!")]
    pub data: String,

    /// Token for authrazation
    #[arg(short, long)]
    pub token : String,
}

pub fn parse_config()->Result<Args,Box<dyn Error>>{

    let args = Args::parse();
    Ok(args)
}