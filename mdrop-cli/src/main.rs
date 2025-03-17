use clap::{Args, Parser, Subcommand};
use mdrop::filter::Filter;
use mdrop::gain::Gain;
use mdrop::indicator_state::IndicatorState;
use mdrop::{volume_level, Moondrop};
use tabled::settings::object::Columns;
use tabled::settings::{Alignment, Style};
use tabled::Table;

#[derive(Debug, Parser)]
#[command(name = "mdrop")]
#[command(about = "A tool to control your Moondrop dongle", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// specify target device, by using the USB bus number, to which the command should be directed, ex. `03:02`
    #[arg(short = 's', global = true)]
    device: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Gets status of Moondrop dongle
    Get(GetArgs),
    /// Sets various values in your Moondrop dongle
    Set(SetArgs),
    /// Lists all the Moondrop dongles connected to the PC
    Devices,
}

#[derive(Debug, Args)]
struct GetArgs {
    #[command(subcommand)]
    command: Option<GetCommands>,
}

#[derive(Debug, Subcommand)]
enum GetCommands {
    /// Gets status for filter, gain, and indicator state
    All,
    /// Gets current hardware volume of Moondrop dongle
    Volume,
}

#[derive(Debug, Args)]
struct SetArgs {
    #[command(subcommand)]
    command: SetCommands,
}

#[derive(Debug, Subcommand)]
enum SetCommands {
    /// Sets audio filter
    Filter { filter: Filter },
    /// Sets gain on device to Low or High
    Gain { gain: Gain },
    /// Sets current hardware volume
    Volume {
        /// Volume level between 0 and 100
        #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
        level: u8,
    },
    /// Sets indicator state to On, Off(temp), or Off
    IndicatorState { state: IndicatorState },
}

fn main() {
    let args = Cli::parse();
    println!("Device: {:?}", args.device);

    let moondrop = Moondrop::new();

    match args.command {
        Commands::Get(get) => {
            let get_cmd = get.command.unwrap_or(GetCommands::All);
            match get_cmd {
                GetCommands::All => {
                    let resp = moondrop.get_all();
                    println!("Filter: {:?}", resp.filter);
                    println!("Gain: {:?}", resp.gain);
                    println!("Indicator State: {:?}", resp.state);
                }
                GetCommands::Volume => {
                    let volume = moondrop.get_volume();
                    println!(
                        "Volume: {}%",
                        volume_level::convert_volume_to_percent(volume)
                    );
                }
            }
        }
        Commands::Set(set) => match set.command {
            SetCommands::Filter { filter } => moondrop.set_filter(filter),
            SetCommands::Gain { gain } => moondrop.set_gain(gain),
            SetCommands::Volume { level } => moondrop.set_volume(level),
            SetCommands::IndicatorState { state } => moondrop.set_indicator_state(state),
        },
        Commands::Devices => {
            let dongles = moondrop.detect();
            if !dongles.is_empty() {
                let table = Table::new(dongles)
                    .with(Style::sharp())
                    .modify(Columns::last(), Alignment::right())
                    .to_string();
                println!("{table}");
            } else {
                println!("No devices present");
            }
        }
    }
}
