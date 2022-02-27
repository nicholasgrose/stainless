use std::time::Duration;

use tokio::io::AsyncReadExt;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;

pub struct ServerControl {
    pub control_thread: JoinHandle<crate::Result<()>>,
    pub control_receiver: Receiver<u8>,
}

pub async fn create_control_socket() -> crate::Result<ServerControl> {
    let (tx, rx) = tokio::sync::mpsc::channel::<u8>(1024);

    let control_task = tokio::spawn(write_input(tx));

    Ok(ServerControl {
        control_thread: control_task,
        control_receiver: rx,
    })
}

async fn write_input(tx: Sender<u8>) -> crate::Result<()> {
    let mut input = tokio::io::stdin();

    loop {
        let byte_read = input.read_u8().await?;

        tx.send(byte_read).await?;
    }
}

pub async fn server_should_stop(socket: &mut Receiver<u8>) -> crate::Result<bool> {
    loop {
        let sleep = tokio::time::sleep(Duration::from_secs(5));

        select! {
            result = should_restart_response(socket) => {
                match result {
                    Ok(response) => if response.is_empty() || response == "y" || response == "yes" {
                        return Ok(false)
                    } else if response == "n" || response == "no" {
                        return Ok(true)
                    } else {
                        println!("Response invalid. Please try again...")
                    },
                    Err(e) => {
                        println!("Error reading response: {}", e);
                        return Ok(false)
                    }
                }
            }
            _ = sleep => {
                println!("No user response acquired in time. Restarting...");
                return Ok(false)
            }
        }
    };
}

async fn should_restart_response(socket: &mut Receiver<u8>) -> crate::Result<String> {
    println!("Restart server? [Y/n]");

    let mut line = Vec::<u8>::new();

    loop {
        line.push(
            match socket.recv().await {
                Some(byte) => byte,
                None => return Err(anyhow::Error::msg("Input channel broke."))
            }
        );

        if line.ends_with(&[b'\n']) {
            return Ok(String::from_utf8(line)?.trim_end().to_string());
        }
    }
}
