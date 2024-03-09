use owo_colors::OwoColorize;
use std::process::Command;

use clap::Args;
use regex::Regex;

use crate::{
    apps,
    utils::{command, env},
};

#[derive(Debug, Args)]
pub struct CommandArgs {
    /// The steam launch command '%command%'
    pub command_args: Option<Vec<String>>,
}

pub fn command(args: CommandArgs) -> Result<(), String> {
    let command_args = args.command_args.unwrap();
    let command_args: Vec<&str> = command_args.iter().map(|x| x.as_str()).collect();
    let command = command::join(command_args)?;

    let re = Regex::new(r"AppId=(?<app_id>\d+)").unwrap();

    if let Some(caps) = re.captures(&command) {
        let app_id = &caps["app_id"];

        println!("App ID: {0}", &caps["app_id"]);

        let app = apps::get(app_id);

        if app.tweaks.tricks.len() > 0 {
            let (app_tweaks_applied, app_total_tweaks) = apps::apply(&app)?;

            if app_tweaks_applied == 0 {
                println!(
                    "{} {}",
                    "No tweaks were necessary!".green().bold(),
                    format!("({app_total_tweaks} tweaks attempted)").italic()
                );
            } else {
                println!(
                    "Applied {} successfully!",
                    format!("{app_tweaks_applied} tweaks").bold()
                );
            }
        }

        let command = command::split(&command)?;

        let mut env = app.tweaks.env;
        if let Some(esync) = app.tweaks.settings.esync {
            env.insert(
                "PROTON_NO_ESYNC".to_string(),
                if esync {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            );
        }

        if let Some(fsync) = app.tweaks.settings.fsync {
            env.insert("PROTON_NO_FSYNC".to_string(), env::convert_bool(fsync));
        }

        Command::new(&command[0])
            .args(&command[1..])
            .envs(env)
            .spawn()
            .expect("Failed to run command")
            .wait()
            .expect("Failed to wait for command");
    } else {
        warn!("Protontweaks purely acts as a passthrough for non-steam games!");
        let command = command::split(&command)?;

        Command::new(&command[0])
            .args(&command[1..])
            .spawn()
            .expect("Failed to run command")
            .wait()
            .expect("Failed to wait for command");
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::{command, CommandArgs};

    #[test]
    pub fn command_should_support_simple_commands() {
        command(CommandArgs {
            command_args: Some(vec!["echo".to_string(), "hello".to_string()]),
        })
        .expect("Failed to execute command.");
    }

    #[test]
    pub fn command_should_support_unified_commands() {
        command(CommandArgs {
            command_args: Some(vec!["echo hello".to_string()]),
        })
        .expect("Failed to execute command.");
    }

    #[test]
    pub fn command_should_support_steam_launch_commands() {
        let command_args = vec![
            "/home/ceci/.local/share/Steam/ubuntu12_32/reaper",
            "SteamLaunch",
            "AppId=644930",
            "--",
            "/home/ceci/.local/share/Steam/ubuntu12_32/steam-launch-wrapper",
            "--",
            "'/home/ceci/.local/share/Steam/steamapps/common/SteamLinuxRuntime_sniper'/_v2-entry-point",
            "--verb=waitforexitandrun",
            "--",
            "'/home/ceci/.local/share/Steam/steamapps/common/Proton 9.0 (Beta)'/proton",
            "waitforexitandrun",
            "'/home/ceci/.local/share/Steam/steamapps/common/They Are Billions/TheyAreBillions.exe'"
        ].iter_mut().map(|x| x.to_string()).collect::<Vec<String>>();

        command(CommandArgs {
            command_args: Some(command_args),
        })
        .expect("Failed to execute command.");
    }
}
