use clap::{App, AppSettings, Arg, SubCommand};

#[derive(Debug)]
pub struct Command {
    pub command_name: &'static str,
    pub command_info: &'static str,
    pub command_actions: &'static [(&'static str, &'static [&'static str])],
    pub after_help: String,
}

impl Command {
    fn new(
        command_name: &'static str,
        command_info: &'static str,
        command_actions: &'static [(&'static str, &'static [&'static str])],
    ) -> Self {
        let mut after_help = "  The possible options for <action> are:\n".to_string();
        for action in command_actions {
            after_help.push_str(&format!("    + {}\n", action.0));
        }

        Command {
            command_name,
            command_info,
            command_actions,
            after_help,
        }
    }
}

pub fn generate_command_definitions() -> Vec<Command> {
    vec![
        Command::new("power", "Commands related to power management", &[("off", &["POWER off"])]),
        Command::new(
            "query",
            "Commands that retrieve some info about the current state of the TV",
            &[
                ("current-app", &["CURRENT_APP"]),
                (
                    "mac-addresses",
                    &["GET_MACADDRESS wired", "GET_MACADDRESS wifi"],
                ),
                ("mute", &["MUTE_STATE"]),
                ("volume", &["CURRENT_VOL"]),
            ],
        ),
        Command::new(
            "screen",
            "Toggle the video source or the video + OSD off and on",
            &[
                ("off", &["SCREEN_MUTE screenmuteon"]),
                ("on", &["SCREEN_MUTE allmuteoff"]),
                ("video-source-off", &["SCREEN_MUTE videomuteon"]),
            ],
        ),
        Command::new(
            "key",
            "Press keys from the remote controller",
/*
exit
sleepreserve
livetv
previouschannel
favoritechannel
teletext
teletextoption
returnback
avmode
captionsubtitle
myapp
settingmenu
ok
quickmenu
videomode
audiomode
channellist
bluebutton
yellowbutto n
greenbutton
redbutton
aspectratio
audiodes cription
programmorder
userguide
smarthome
simplelink
fastforward
rewind
programminfo
programguide
play
slowplay
soccerscreen
reord
3d
autoconfig
app
screenbright
*/
            &[
                ("1", &["KEY_ACTION number1"]),
                ("2", &["KEY_ACTION number2"]),
                ("3", &["KEY_ACTION number3"]),
                ("4", &["KEY_ACTION number4"]),
                ("5", &["KEY_ACTION number5"]),
                ("6", &["KEY_ACTION number6"]),
                ("7", &["KEY_ACTION number7"]),
                ("8", &["KEY_ACTION number8"]),
                ("9", &["KEY_ACTION number9"]),
                ("0", &["KEY_ACTION number0"]),
                ("mute", &["KEY_ACTION volumemute"]),
                ("input-list", &["KEY_ACTION deviceinput"]),
                ("left", &["KEY_ACTION arrowleft"]),
                ("right", &["KEY_ACTION arrowright"]),
                ("vol-up", &["KEY_ACTION volumeup"]),
                ("vol-down", &["KEY_ACTION volumedown"]),
                ("channel-up", &["KEY_ACTION channelup"]),
                ("channel-down", &["KEY_ACTION channeldown"]),
            ],
        ),
        Command::new(
            "input",
            "Change the current input of the TV",
            &[
                ("hdmi-1", &["APP_LAUNCH com.webos.app.hdmi1"]),
                ("hdmi-2", &["APP_LAUNCH com.webos.app.hdmi2"]),
                ("hdmi-3", &["APP_LAUNCH com.webos.app.hdmi3"]),
                ("hdmi-4", &["APP_LAUNCH com.webos.app.hdmi4"]),
                ("netflix", &["APP_LAUNCH netflix"]),
                ("youtube", &["APP_LAUNCH youtube.leanback.v4"]),
            ],
        ),
        Command::new(
            "aspect-ratio",
            "Change the aspect ratio of the TV",
            &[
                ("standard", &["ASPECT_RATIO 4by3"]),
                ("wide", &["ASPECT_RATIO 16by9"]),
                ("4by3", &["ASPECT_RATIO 4by3"]),
                ("16by9", &["ASPECT_RATIO 16by9"]),
                ("keep-unchanged", &["ASPECT_RATIO setbyoriginal"]),
            ],
        ),
        Command::new(
            "set",
            "Commands to change the value of settings on the TV",
            &[
                ("mute", &["VOLUME_MUTE {}"]),
                ("volume", &["VOLUME_CONTROL {}"]),
                ("backlight", &["PICTURE_BACKLIGHT {}"]),
            ]
        ),
        Command::new("custom", "To send any raw custom command directly", &[("command", &["{}"])]),
    ]
}

pub fn generate_clap_subcommands<'a>(commands: &'a [Command]) -> Vec<App<'a, 'a>> {
    commands
        .iter()
        .map(|command| {
            SubCommand::with_name(command.command_name)
                .long_about(command.command_info)
                .setting(AppSettings::TrailingVarArg)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::DisableHelpFlags)
                .after_help(&*command.after_help)
                .arg(
                    Arg::with_name("action")
                        .required(true)
                        .possible_values(
                            &command
                                .command_actions
                                .iter()
                                .map(|(k, _)| *k)
                                .collect::<Vec<&str>>(),
                        )
                        .hide_possible_values(true),
                )
                .arg(Arg::with_name("parameters").multiple(true))
        })
        .collect()
}
