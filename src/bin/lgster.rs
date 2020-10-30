use std::convert::TryInto;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use lgster::cli::{generate_command_definitions, get_parameters};
use lgster::comm::send_command;

fn main() -> Result<(), Box<dyn Error>> {
    let command_definitions = generate_command_definitions();
    let params = get_parameters(&command_definitions)?;
    let (command, action) = (params.command, params.command_action);

    // TODO: better feedback to the user
    let command_actions = command
        .command_actions
        .iter()
        .find(|a| a.0 == action)
        .expect("Invalid command");

    let target_address = params.host.unwrap();
    let salt: [u8; 16] = params.salt.0.bytes[..]
        .try_into()
        .expect("Invalid salt size. Should be 16 bytes.");
    for action_to_run in command_actions.1 {
        let action_to_run = action_to_run.replace("{}", &params.command_action_parameters[0]);
        let response = match send_command(
            &target_address,
            params.port,
            &params.keycode,
            &salt,
            action_to_run,
        ) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}", e);
                return Ok(());
            }
        };
        println!(
            "{}",
            response
                .chars()
                .take_while(|c| *c != '\n')
                .collect::<String>()
        );
        sleep(Duration::from_millis(200));
    }
    Ok(())
}
