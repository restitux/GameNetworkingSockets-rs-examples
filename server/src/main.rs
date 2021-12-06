use gamenetworkingsockets_rs as gns;

use eyre::Result;

fn main() -> Result<()> {

    let mut sns = gns::init()?;

    let ip = "127.0.0.1:32320".parse()?;
    let listen_sock = sns.create_listen_socket_ip(ip, None)?;

    let poll_group = sns.create_poll_group()?;

    println!("Server listening on {}...", ip);

    loop {
        if let Some(messages) = sns.receive_messages_on_poll_group(&poll_group)? {
            println!("Received {} messages", messages.msgs.len());
        }

        sns.run_callbacks();

        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    sns.close_listen_socket(listen_sock)?;
    sns.destroy_poll_group(poll_group)?;

    Ok(())
}
