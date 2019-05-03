extern crate svgcode;
#[macro_use]
extern crate structopt;

use svgcode::input::Parser;
use svgcode::output::Exporter;
use structopt::StructOpt;
use std::path::PathBuf;
use svgcode::input::svg::SvgParser;
use svgcode::output::gcode::GcodeExporter;

#[derive(StructOpt, Debug)]
struct Cmdline{
    /// Input file
    #[structopt(short="i", long="input",  parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(short="o", long="output", parse(from_os_str))]
    output: PathBuf,

    /// Input format
    #[structopt(short="f", long="format", default_value="svg")]
    format: String
}

fn main() -> Result<(), std::io::Error> {
    let cmdline = Cmdline::from_args();

    println!("Miau-starting");

    if let Ok(program) = SvgParser::new(&cmdline.input).parse(){
        GcodeExporter::new(&cmdline.output).export(program)
    } else {
        Ok(())
    }


}
