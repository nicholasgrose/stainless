use std::path::PathBuf;
use std::time::Duration;

use tempfile::TempDir;
use tokio::net::UnixDatagram;
use tokio::select;
use tokio::task::JoinHandle;

pub struct ServerControl {
    pub control_thread: JoinHandle<crate::Result<()>>,
    pub control_receiver: UnixDatagram,
}

pub async fn create_control_socket(temp_dir: &TempDir) -> crate::Result<ServerControl> {
    let tx_path = temp_dir.path().join("tx");
    let tx = UnixDatagram::bind(&tx_path)?;
    let rx_path = temp_dir.path().join("rx");
    let rx = UnixDatagram::bind(&rx_path)?;

    let control_task = tokio::spawn(write_input(tx, rx_path));

    Ok(ServerControl {
        control_thread: control_task,
        control_receiver: rx,
    })
}

async fn write_input(tx: UnixDatagram, rx_path: PathBuf) -> crate::Result<()> {
    let input = std::io::stdin();
    let mut buffer = String::new();

    loop {
        let bytes_read = input.read_line(&mut buffer)?;
        let bytes_to_write = &buffer.as_bytes()[..bytes_read];

        tx.send_to(bytes_to_write, &rx_path).await?;
    }
}

pub async fn server_should_stop(socket: &UnixDatagram) -> crate::Result<bool> {
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

async fn should_restart_response(socket: &UnixDatagram) -> crate::Result<String> {
    println!("Restart server? [Y/n]");

    let mut buffer = vec![0; 1024];
    let (bytes_received, _) = socket.recv_from(&mut buffer).await?;

    let line = String::from_utf8_lossy(&buffer[..bytes_received])
        .trim_end()
        .to_lowercase();

    println!("GOT: {}", line);

    Ok(line)
}
