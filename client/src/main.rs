use gamenetworkingsockets_rs as gns;

use eyre::Result;

fn main() -> Result<()> {

    let mut sns = gns::init()?;

    let ip = "127.0.0.1:32320".parse()?;

    println!("Connecting to {}...", ip);

    let net_connection = sns.connect_by_ip_address(ip, None)?;

    println!("Connected to {}", ip);


    loop {
        sns.run_callbacks();

        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    Ok(())
}
