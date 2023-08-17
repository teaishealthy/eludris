use std::{env, str::FromStr};

use anyhow::{bail, Context};
use console::Style;
use dialoguer::{theme, Confirm, Editor, Input};
use eludris::{
    check_eludris_exists, download_file, end_progress_bar, get_user_config, new_docker_command,
    new_progress_bar,
};
use reqwest::Client;
use todel::Conf;
use tokio::fs;

use crate::clean;

pub async fn deploy(next: bool) -> anyhow::Result<()> {
    let config = get_user_config()
        .await?
        .context("Could not find user config")?;

    if !check_eludris_exists(&config)? {
        let bar = new_progress_bar("Eludris directory not found, setting up...");

        let client = Client::new();
        download_file(
            &config,
            &client,
            "docker-compose.prebuilt.yml",
            next,
            Some("docker-compose.yml"),
        )
        .await?;
        download_file(&config, &client, "docker-compose.override.yml", next, None).await?;
        download_file(&config, &client, ".env.example", next, Some(".env")).await?;
        download_file(
            &config,
            &client,
            "Eludris.example.toml",
            next,
            Some("Eludris.toml"),
        )
        .await?;
        fs::create_dir(format!("{}/files", config.eludris_dir))
            .await
            .context("Could not create effis files directory")?;
        end_progress_bar(bar, "Finished setting up instance files");

        let editor: String;
        loop {
            let editor_input = Input::with_theme(&theme::ColorfulTheme {
                prompt_prefix: Style::new().yellow().bold().apply_to("~>".to_string()),
                success_prefix: Style::new().green().bold().apply_to("~>".to_string()),
                error_prefix: Style::new().red().bold().apply_to("~>".to_string()),
                ..Default::default()
            })
            .with_prompt(
                "Please enter the name of your preferred editor or command to start said editor",
            )
            .default(env::var("EDITOR").unwrap_or_else(|_| "vi".to_string()))
            .interact_text()
            .context("Could not prompt user")?;

            editor = match editor_input.trim() {
                "code" | "vscode" | "vsc" | "vscodium" => {
                    println!("Using vscode or any vscode based editor is {} recommended as it can screw up a lot of stuff while running as root. Try something else", Style::new().bold().apply_to("not"));
                    continue;
                }
                "neovide" => "neovide --nofork".to_string(),
                _ => editor_input,
            };
            break;
        }

        let mut base_conf = fs::read_to_string(format!("{}/Eludris.toml", config.eludris_dir))
            .await
            .context("Could not read Eludris.toml file")?;
        loop {
            let conf = Editor::new()
                .executable(&editor)
                .extension("toml")
                .require_save(true)
                .trim_newlines(false)
                .edit(&base_conf)
                .context("Could not setup editor")?;
            match conf {
                Some(conf_string) => {
                    base_conf = conf_string.clone();
                    let conf = Conf::from_str(&conf_string);
                    match conf {
                        Ok(_) => {
                            fs::write(format!("{}/Eludris.toml", config.eludris_dir), conf_string)
                                .await
                                .context("Could not write new config to Eludris.toml")?;
                            break;
                        }
                        Err(err) => {
                            if !Confirm::with_theme(&theme::ColorfulTheme::default())
                                .with_prompt(format!("{}, try again", err.to_string().trim()))
                                .interact()
                                .context("Could not spawn confirm prompt")?
                            {
                                clean::clean().await?;
                                bail!("Operation cancelled");
                            };
                            continue;
                        }
                    }
                }
                None => {
                    if !Confirm::with_theme(&theme::ColorfulTheme::default())
                        .with_prompt("Error: Please configure your instance")
                        .interact()
                        .context("Could not spawn confirm prompt")?
                    {
                        clean::clean().await?;
                        bail!("Operation cancelled");
                    };
                    continue;
                }
            }
        }

        if Confirm::with_theme(&theme::ColorfulTheme::default())
            .with_prompt("Do you want to configure your instance's `.env` file?")
            .interact()
            .context("Could not spawn confirm prompt")?
        {
            let base_env = fs::read_to_string(format!("{}/.env", config.eludris_dir))
                .await
                .context("Could not read .env file")?;
            if let Some(env) = Editor::new()
                .executable(&editor)
                .require_save(true)
                .trim_newlines(false)
                .edit(&base_env)
                .context("Could not setup editor")?
            {
                fs::write(format!("{}/.env", config.eludris_dir), env)
                    .await
                    .context("Could not write new config to .env")?;
            }
        };

        println!(
            "{}",
            Style::new()
                .green()
                .apply_to("Great, you've succesfully setup your own Eludris instance")
        );
    }

    let command = new_docker_command(&config)
        .arg("up")
        .arg("-d")
        .spawn()
        .context("Could not start instance, make sure you have docker-compose installed")?
        .wait()
        .await
        .context("Instance failed to start")?;

    if command.success() {
        println!(
            "{}",
            Style::new().green().apply_to(
                "Instance sucessfully started, now test it out with pilfer or using curl"
            )
        );
    }

    Ok(())
}
