use std::error::Error;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use uuid::Uuid;
use tokio::time;
use tokio::time::{sleep, Duration};


async fn find_device(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("Smart 2.0"))
        {
            return Some(p);
        }
    }
    None
}

fn chksum(data: &[u8]) -> u8 {
    let mut sum = 0;
    for i in 1..data.len() - 1 {
        sum += u16::from(data[i]);
    }
    return ((0x100 - (sum & 0xFF)) & 0xFF) as u8;
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // service and characteristic have the same uuid for the brio smart 2.0
    let service_id = Uuid::parse_str("B11B0002-BF9B-4A20-BA07-9218FEC577D7").unwrap();

    println!("Initializing BLE manager");
    let manager = Manager::new().await.unwrap();
    // Get the first bluetooth adapter
    let adapters = manager.adapters().await.unwrap();
    let central = adapters.into_iter().next().unwrap();
    
    print!("Scanning for devices with service ID: {}\n", service_id);
    central.start_scan(ScanFilter::default()).await.unwrap();
    // Wait a bit to collect some devices
    time::sleep(Duration::from_secs(2)).await;

    let timeout = Duration::from_secs(30);
    let start = std::time::Instant::now();

    let mut temp_device  = None;
    
    while start.elapsed() < timeout {
        if let Some(d) = find_device(&central).await {
            println!("Device found!");
            temp_device = Some(d);
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    let device = temp_device.expect("No expected device found");
    
    println!("Connecting to device");
    device.connect().await?;
    device.discover_services().await?;

    let chars = device.characteristics();
    let cmd_char = chars.iter().find(|c| c.uuid == service_id).expect("Could not find command characteristic");

    println!("Writing data");

    // sending different colors
    for c in 1..255 {
        let mut cmd: Vec<u8> = vec![0xAA, 0x02, 0x02, c, 0x00];
        let sum = chksum(&cmd);
        cmd[4] = sum;
        device.write(&cmd_char, &cmd, WriteType::WithoutResponse).await?;
        time::sleep(Duration::from_millis(70)).await;
    }

    let off_cmd: Vec<u8> = vec![0xAA, 0x02, 0x02, 0x00 , 0xFC];
    device.write(&cmd_char, &off_cmd, WriteType::WithoutResponse).await?;
    // we have to wait he a bit to make sure the command is sent
    time::sleep(Duration::from_millis(300)).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chksum() {
        let blue_cmd= vec![0xAA, 0x02, 0x02, 0x7A, 0x82];
        let light_blue_cmd= vec![0xAA, 0x02, 0x02, 0x6A, 0x92];
        let off_cmd = vec![0xAA, 0x02, 0x02, 0x00 , 0xFC];

        assert_eq!(chksum(&blue_cmd), 0x82);
        assert_eq!(chksum(&light_blue_cmd), 0x92);
        assert_eq!(chksum(&off_cmd), 0xFC);
    }
}