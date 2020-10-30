use std::error::Error;
use std::fmt;
use std::str::FromStr;
use clap::{App, AppSettings, Arg, ArgMatches};

use super::commands::{Command, generate_clap_subcommands};
use super::convert::FixedSizeByteSequenceParameter;

fn get_matches<'a, T: IntoIterator<Item=App<'a, 'a>>>(subcommands: T) -> ArgMatches<'a> {
    App::new("lgster")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::DeriveDisplayOrder)
        .version("0.1.0")
        .author("Mateus \"Doodad\" Medeiros <dood.ad@outlook.com>")
        .about("Wrapper around the LG ip control function to control TVs remotely over the network.")
        .arg(
            Arg::with_name("Keycode")
                .short("k")
                .long("keycode")
                .value_name("ABCDEFGH")
                .next_line_help(true)
                .long_help(concat!(
                    "The keycode generated by the TV when enabling the IP Control function", "\n",
                    "\n",
                    "This must be the same keycode that your TV generated and is a required parameter.", "\n",
                    "It is used as a shared secret between you and the TV and the message with the command", "\n",
                    "is encrypted with AES-128 CBC with a key derived from it."
                ))
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("Target host")
                .short("t")
                .long("target-host")
                .value_name("x.x.x.x")
                .next_line_help(true)
                .long_help(concat!(
                    "The host of your LG TV on the network", "\n",
                    "  e.g. 192.168.0.2", "\n",
                    "", "\n",
                    "If this parameter is not specified, the encrypted message will only be output to the stdout.", "\n",
                    "The output can then be transmitted through netcat or something similar.", "\n",
                    "Note that in that case, you will have to handle the response yourself (or ignore it).", "\n",
                    "For completely silent output (besides the message) use the --quiet parameter. If you", "\n",
                    "don't, other output will be output through stderr so that the stdout can be redirected.", "\n",
                    "", "\n",
                    "NOTE: Don't append the port to the host. Use the parameter -p if you want to change the default port (9761)."
                ))
                .takes_value(true)
        )
        .arg(
            Arg::with_name("Target port")
                .short("p")
                .long("target-port")
                .value_name("9761")
                .next_line_help(true)
                .long_help(concat!(
                    "(You shouldn't usually need to change this)", "\n",
                    "", "\n",
                    "The port of your LG TV on the network", "\n",
                    "The default port is 9761, which is the default port used by", "\n",
                    "many of LG TVs' IP Control service.", "\n",
                ))
                .takes_value(true)
                .default_value("9761")
        )
        .arg(
            Arg::with_name("Salt")
                .long("salt")
                .value_name("00-aa-bb-cc-dd-ee-ff-de-ad-be-ef-d0-0d-ad-00")
                .next_line_help(true)
                .long_help(concat!(
                    "(You shouldn't usually need to change this)", "\n",
                    "", "\n",
                    "A string of hyphen-separated hex-represented bytes to be used as the salt to generate ", "\n",
                    "the encryption key with the provided keycode."
                ))
                .takes_value(true)
                .default_value("63-61-b8-0e-9b-dc-a6-63-8d-07-20-f2-cc-56-8f-b9")
        )
        .arg(
            Arg::with_name("Custom IV")
                .long("iv")
                .value_name("00-00-00-00-00-00-00-00-00-00-00-00-00-00-00-00")
                .next_line_help(true)
                .long_help(concat!(
                    "(You shouldn't usually need to change this)", "\n",
                    "", "\n",
                    "A string of hex digits used as the IV to encrypt the message itself", "\n",
                    "By default it is randomly generated.", "\n",
                ))
                .takes_value(true)
        )
        .arg(
            Arg::with_name("Quiet mode")
                .short("q")
                .long("quiet")
                .takes_value(false)
                .next_line_help(true)
                .long_help(concat!(
                    "Disables all console output", "\n",
                    "", "\n",
                    "If --target-host is unspecified, though, the encrypted message itself", "\n",
                    "(and nothing more) will be output.", "\n",
                ))
        )
        .arg(
            Arg::with_name("Debug")
                .short("d")
                .long("debug")
                .takes_value(false)
                .help("Enables verbose output of each step")
                .next_line_help(true)
                .overrides_with("Quiet mode")
        )
        .subcommands(subcommands)
        .get_matches()
}

#[derive(Debug)]
pub struct Salt(pub FixedSizeByteSequenceParameter);

#[derive(Debug)]
pub struct IV(pub FixedSizeByteSequenceParameter);

#[derive(Debug)]
pub struct ParseParameterError {
    pub parameter_name: String,
    source_error: Box<dyn Error>
}

impl fmt::Display for ParseParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while converting parameter {} ({})", self.parameter_name, self.source_error)
    }
}

impl Error for ParseParameterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.source_error)
    }
}

impl FromStr for Salt {
    type Err = ParseParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let param = FixedSizeByteSequenceParameter::from_string(s.to_string(), '-', 16);
        match param {
            Ok(p) => Ok(Salt(p)),
            Err(e) => Err(ParseParameterError { parameter_name: "--salt".to_string(), source_error: e })
        }
    }
}

impl FromStr for IV {
    type Err = ParseParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let param = FixedSizeByteSequenceParameter::from_string(s.to_string(), '-', 16);
        match param {
            Ok(p) => Ok(IV(p)),
            Err(e) => Err(ParseParameterError { parameter_name: "--iv".to_string(), source_error: e })
        }
    }
}

#[derive(Debug)]
pub struct Parameters<'a> {
    pub keycode: String,
    pub host: Option<String>,
    pub port: u16,
    pub salt: Salt,
    pub iv: Option<IV>,
    pub quiet: bool,
    pub debug: bool,
    pub command: &'a Command,
    pub command_action: String,
    pub command_action_parameters: Vec<String>,
}

impl Parameters<'_> {
    fn try_from_matches<'a>(matches: ArgMatches<'a>, commands: &'a [Command]) -> Result<Parameters<'a>, ParseParameterError> {
        // this is safe to unwrap because it's required so clap will validate that for us
        let keycode: String = matches.value_of("Keycode").unwrap().to_string();
        let host = matches.value_of("Target host").map(str::to_string);

        // this is safe to unwrap because it has a default value, even if the user don't specify one
        let port = matches.value_of("Target port").unwrap().parse::<u16>().map_err(|e|
            ParseParameterError { parameter_name: "--target-port".to_string(), source_error: Box::new(e) }
        )?;
        let salt = Salt::from_str(matches.value_of("Salt").unwrap())?;
        let iv = match matches.value_of("Custom IV") {
            Some(iv_string) => Some(IV::from_str(iv_string)?),
            None => None
        };
        let quiet = matches.is_present("Quiet mode");
        let debug = matches.is_present("Debug");

        // these are safe to unwrap because we're setting SubcommandRequiredElseHelp
        let command_name = matches.subcommand_name().unwrap();
        let subcommand_matches = matches.subcommand_matches(command_name).unwrap();
        // this is safe to unwrap because our action arg of every clap SubCommand is always required
        let command_action = subcommand_matches.value_of("action").unwrap().to_string();
        let command = commands
            .iter()
            .find(|c| c.command_name == command_name)
            .expect("Missing command def. This should never happen!");

        // this is safe to unwrap because if it is not used, it will be an empty vec, but not an error
        let command_action_parameters: Vec<String> = subcommand_matches
            .values_of("parameters")
            .unwrap()
            .map(String::from)
            .collect();

        Ok(Parameters { keycode, host, port, salt, iv, quiet, debug, command, command_action, command_action_parameters })
    }
}

pub fn get_parameters<'a>(commands: &'a[Command]) -> Result<Parameters<'a>, ParseParameterError> {
    Parameters::try_from_matches(get_matches(generate_clap_subcommands(&commands)), &commands)
}
