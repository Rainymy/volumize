use serialport::{SerialPortType, UsbPortInfo};

struct PortInfo {
    name: String,
    info: UsbPortInfo,
}

fn get_available_devices() -> Vec<PortInfo> {
    let ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(_) => Vec::new(),
    };

    println!("Available ports: {}", ports.len());

    let mut result = vec![];
    for p in ports {
        println!("Port: {}", p.port_name);
        match p.port_type {
            SerialPortType::UsbPort(info) => {
                result.push(PortInfo {
                    name: p.port_name,
                    info: info,
                });
            }
            SerialPortType::PciPort => println!("  Type: PCI"),
            SerialPortType::BluetoothPort => println!("  Type: Bluetooth"),
            SerialPortType::Unknown => println!("  Type: Unknown"),
        }
    }

    result
}

fn find_device() -> Option<PortInfo> {
    use shared_types::info::SERIAL_NUMBER;
    let devices = get_available_devices();

    devices
        .into_iter()
        .find(|d| d.info.serial_number == Some(SERIAL_NUMBER.to_string()))
}
