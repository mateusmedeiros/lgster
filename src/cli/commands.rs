use clap::{App, AppSettings, Arg, SubCommand};

#[derive(Debug)]
pub struct Command {
    pub command_name: &'static str,
    pub command_info: &'static str,
    pub command_actions: &'static [(&'static str, &'static [&'static str])],
    pub after_help: String
}

impl Command {
    fn new(command_name: &'static str, command_info: &'static str, command_actions: &'static [(&'static str, &'static [&'static str])]) -> Self {
        let mut after_help = "  The possible options for <action> are:\n".to_string();
        for action in command_actions {
            after_help.push_str(&format!("    + {}\n", action.0));
        }

        Command { command_name, command_info, command_actions, after_help }
    }
}

pub fn generate_command_definitions() -> Vec<Command> {
    vec![
        Command::new("power", "Send stuff", &[
            ("off", &["POWER off"]),
        ]),
        Command::new("query", "Send stuff", &[
            ("current-app", &["CURRENT_APP"]),
            ("mac-addresses", &["GET_MACADDRESS wired", "GET_MACADDRESS wifi"]),
            ("mute", &["MUTE_STATE"]),
            ("volume", &["CURRENT_VOL"]),
        ]),
        Command::new("set", "Send stuff", &[
            ("volume", &["VOLUME_CONTROL {}"])
        ]),
        Command::new("custom", "Send stuff", &[
            ("command", &["{}"])
        ])
    ]
}

pub fn generate_clap_subcommands<'a>(commands: &'a [Command]) -> Vec<App<'a, 'a>> {
    commands.iter().map(|command| {
        SubCommand::with_name(command.command_name)
            .long_about(command.command_info)
            .setting(AppSettings::TrailingVarArg)
            .setting(AppSettings::DisableVersion)
            .setting(AppSettings::DisableHelpFlags)
            .after_help(&*command.after_help)
            .arg(
                Arg::with_name("action")
                    .required(true)
                    .possible_values(&command.command_actions.iter().map(|(k, _)| *k).collect::<Vec<&str>>())
                    .hide_possible_values(true)
            )
            .arg(
                Arg::with_name("parameters")
                    .multiple(true)
            )
    }).collect()
}
