use structopt::StructOpt;
use rust_gr::data::{load_parameter_data, load_csv_data, save_flow};
use rust_gr::models::GR4JModel;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    parameters: std::path::PathBuf,
    data: std::path::PathBuf,
    output_path: std::path::PathBuf,
}

fn main () {
    println!("Hello World");

    let args = Cli::from_args();

    let data = load_csv_data(args.data);
    let parameters = load_parameter_data(args.parameters);

    let mut model = GR4JModel::new(parameters);

    let flow = model.run(data.rainfall, data.pet);
    //println!("{:?}", flow)
    save_flow(data.dates, flow, args.output_path);
    
}