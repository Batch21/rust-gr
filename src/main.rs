use rust_gr::data::{load_csv_data, load_parameter_data, save_flow};
use rust_gr::models::GR4JModel;
use structopt::StructOpt;
#[derive(StructOpt)]
 pub struct Cli {
    #[structopt(parse(from_os_str))]
    parameters: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    data: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    output_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let data = load_csv_data(args.data).expect("Failed to load data file");
    let parameters = load_parameter_data(args.parameters).expect("Failed to load Parameters file");

    let mut model = GR4JModel::new(parameters);

    let flow = model.run(data.rainfall, data.pet);

    if let Err(err) = save_flow(data.dates, flow, args.output_path) {
        println!("Failed to save flow data due to following error '{}'", err);
    };
}