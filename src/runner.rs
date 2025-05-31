use crate::config::{Config, Host};
use crate::output::get_colored_prefix;
use std::process::Stdio;
use tokio::{io::{AsyncBufReadExt, BufReader}, process::Command};

pub async fn run_all(config: &Config, verbose: bool) {
    let mut tasks = vec![];

    for (i, host_config) in config.hosts.iter().enumerate() {
        let command = config.command.clone();
        let host_clone = host_config.clone();
        let prefix = get_colored_prefix(&host_clone.host, i);

        let task = tokio::spawn(async move {
            run_ssh(&host_clone, &command, &prefix, verbose).await;
        });

        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }
}

async fn run_ssh(host: &Host, command: &str, prefix: &str, verbose: bool) {
    let port = host.port.unwrap_or(22);
    let user = &host.user;
    let identity_file = &host.identity_file;

    if verbose {
        println!("{} Connecting to {}:{}...", prefix, host.host, port);
    }

    let mut ssh_args = vec!["-p".to_string(), port.to_string()];

    if let Some(identity) = identity_file {
        ssh_args.push("-i".to_string());
        ssh_args.push(identity.clone());
    }

    let destination = if let Some(user) = user {
        format!("{}@{}", user, host.host)
    } else {
        host.host.clone()
    };

    ssh_args.push(destination);
    ssh_args.push(command.to_string());

    let mut child = match Command::new("ssh")
        .args(&ssh_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("{} Failed to spawn ssh command: {}", prefix, e);
            return;
        }
    };

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            println!("{} {}", prefix, line);
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            eprintln!("{} {}", prefix, line);
        }
    }
}
