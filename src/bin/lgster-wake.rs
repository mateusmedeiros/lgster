use clap::{App, Arg};
use std::error::Error;

use lgster::comm::send_wol_packet;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("lgster-wake")
        .version("1.0")
        .author("Mateus \"Doodad\" Medeiros <dood.ad@outlook.com>")
        .about("Super simple wol wrapper to complement lgster LG IP Control functionalities.")
        .arg(
            Arg::with_name("Target IP address")
                .short("t")
                .long("target-ip")
                .value_name("192.168.0.255")
                .help("The target of the wake-on-lan packet. It's usually set to the broadcast of your subnet.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("Target MAC address")
                .short("m")
                .long("mac-address")
                .value_name("DO:OD:AD:15:DE:AD")
                .help("The MAC Address of your TV.")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    let target_ip = matches.value_of("Target IP address").unwrap();
    let target_mac_address = matches.value_of("Target MAC address").unwrap();
    send_wol_packet(target_ip, target_mac_address)?;
    Ok(())
}
