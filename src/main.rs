extern crate svgcode;

use svgcode::input::Parser;
use svgcode::output::Exporter;
use std::path::PathBuf;
use svgcode::input::svg::SvgParser;
use svgcode::output::gcode::GcodeExporter;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cmdline{
    /// Input file
    #[arg(short='i', long="input")]
    input: PathBuf,

    /// Output file
    #[arg(short='o', long="output")]
    output: PathBuf,

    /// Input format
    #[arg(short='f', long="format", default_value="svg")]
    format: String
}

fn main() -> Result<(), std::io::Error> {
    use clap::Parser;
    let cmdline = Cmdline::parse();

    println!("Miau-starting");

    if let Ok(program) = SvgParser::new(&cmdline.input).parse(){
        GcodeExporter::new(&cmdline.output).export(program)
    } else {
        Ok(())
    }


}
