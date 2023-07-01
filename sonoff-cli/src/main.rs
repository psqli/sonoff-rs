use anyhow::{Result, Context};
use clap::{Parser, Subcommand};

use sonoff_lib::device::SonoffDevice;
use sonoff_lib::bulb::SonoffBulb;
use sonoff_lib::bulb::DevReqBulbColorType;
use sonoff_lib::switch::SonoffSwitch;
use sonoff_lib::dimmer::SonoffDimmer;

use sonoff_lib::switchable::SonoffSwitchable;
use sonoff_lib::dimmable::SonoffDimmable;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Debug mode
    #[arg(long, default_value_t = false)]
    debug: bool,
    /// Address of device
    address: String,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Get information about the device
    Info,
    /// Set device Wi-Fi network
    Wifi {
        ssid: String,
        password: String,
    },
    /// Get or set switch state on switchable devices
    Switch {
        #[command(subcommand)]
        switch_cmd: Option<SwitchCommand>,
    },
    /// Get or set settings on bulb devices
    Bulb {
        #[command(subcommand)]
        bulb_cmd: Option<BulbCommand>,
    },
    Dimmer {
        #[command(subcommand)]
        dimmer_cmd: Option<DimmerCommand>,
    },
}

#[derive(Subcommand)]
enum SwitchCommand {
    On,
    Off,
    Toggle,
    Get,
    Pulse {
        milliseconds: u32,
    },
    /// Set startup state ("on", "off", or "stay")
    Startup {
        startup: String,
    }
}


#[derive(Subcommand)]
enum BulbCommand {
    On,
    Off,
    Toggle,
    Get,
    Rgb {
        brightness: u8,
        red: u8,
        green: u8,
        blue: u8,
    },
    White {
        brightness: u8,
        temperature: u8,
    },
}

#[derive(Subcommand)]
enum DimmerCommand {
    On,
    Off,
    Toggle,
    Get,
    Dim {
        brightness: u8
    },
    /// Set startup state ("on", "off", or "stay")
    Startup {
        startup: String,
    }
}

async fn get_info(dev: &SonoffDevice) -> Result<()> {
    let dev_info = dev.get_info().await?;
    println!("deviceid={}", dev_info.deviceid);
    println!("ssid={}", dev_info.ssid.unwrap_or_default());
    println!("bssid={}", dev_info.bssid.unwrap_or_default());
    println!("signal_strength={}", dev_info.signal_strength.unwrap_or_default());
    println!("fw_version={}", dev_info.fw_version.unwrap_or_default());
    println!("ota_unlock={}", dev_info.ota_unlock.unwrap_or_default());
    Ok(())
}

async fn cli() -> Result<()> {
    let args = Cli::parse();
    let dev = SonoffDevice::new(&args.address);
    let cmd = args.command.context("No command")?;
    match cmd {
        Command::Info => get_info(&dev).await?,
        Command::Wifi { ssid, password } => {
            dev.set_wifi(ssid, password).await?;
        },
        Command::Switch { switch_cmd } => {
            let switch = SonoffSwitch::from(&dev);
            match switch_cmd.context("Invalid switch command")? {
                SwitchCommand::On => { switch.on().await?; },
                SwitchCommand::Off => { switch.off().await?; },
                SwitchCommand::Toggle => { switch.toggle().await?; },
                SwitchCommand::Get => {
                    println!("{}", switch.get_switch().await?);
                },
                SwitchCommand::Pulse { milliseconds } => {
                    switch.pulse(milliseconds).await?;
                }
                SwitchCommand::Startup { startup } => {
                    switch.set_startup(startup).await?;
                },
            }
        },
        Command::Bulb { bulb_cmd } => {
            let bulb = SonoffBulb::from(&dev);
            match bulb_cmd.context("Invalid bulb command")? {
                BulbCommand::On => { bulb.on().await?; },
                BulbCommand::Off => { bulb.off().await?; },
                BulbCommand::Toggle => { bulb.toggle().await?; },
                BulbCommand::Get => {
                    let bulb_info = bulb.get_info().await?;
                    println!("switch={}", bulb_info.switch);
                    match bulb_info.color_type {
                        DevReqBulbColorType::Color(rgb_bulb_info) => {
                            println!("brightness={}", rgb_bulb_info.br);
                            println!("red={}", rgb_bulb_info.r);
                            println!("green={}", rgb_bulb_info.g);
                            println!("blue={}", rgb_bulb_info.b);
                        },
                        DevReqBulbColorType::White(white_bulb_info) => {
                            println!("brightness={}", white_bulb_info.br);
                            println!("temperature={}", white_bulb_info.ct);
                        },
                    }
                },
                BulbCommand::Rgb { brightness, red, green, blue } => {
                    bulb.color(brightness, red, green, blue).await?;
                },
                BulbCommand::White { brightness, temperature } => {
                    bulb.white(brightness, temperature).await?;
                },
            }
        },
        Command::Dimmer { dimmer_cmd } => {
            let dimmer = SonoffDimmer::from(&dev);
            match dimmer_cmd.context("Invalid dimmer command")? {
                DimmerCommand::On => { dimmer.on().await?; },
                DimmerCommand::Off => { dimmer.off().await?; },
                DimmerCommand::Toggle => { dimmer.toggle().await?; },
                DimmerCommand::Get => {
                    let dimmer_info = dimmer.get_info().await?;
                    println!("switch={}", dimmer_info.switch);
                    println!("brightness={}", dimmer_info.brightness);
                },
                DimmerCommand::Dim { brightness } => {
                    dimmer.dim(brightness).await?;
                },
                DimmerCommand::Startup { startup } => {
                    dimmer.set_startup(startup).await?;
                }
            }
        },
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    cli().await
}
