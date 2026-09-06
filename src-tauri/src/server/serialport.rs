use tokio_serial::{SerialPortType, UsbPortInfo};

pub struct UsbPort {
    pub name: String,
    pub info: UsbPortInfo,
}

fn get_available_devices() -> Vec<UsbPort> {
    let ports = match tokio_serial::available_ports() {
        Ok(ports) => ports,
        Err(_) => Vec::new(),
    };

    println!("Available ports: {}", ports.len());

    let mut result = vec![];
    for p in ports {
        println!("Port: {}", p.port_name);
        match p.port_type {
            SerialPortType::UsbPort(info) => result.push(UsbPort {
                name: p.port_name,
                info: info,
            }),
            SerialPortType::PciPort => {}
            SerialPortType::BluetoothPort => {}
            SerialPortType::Unknown => {}
        }
    }

    result
}

pub fn find_devices() -> Vec<UsbPort> {
    use shared_types::info::SERIAL_NUMBER;
    let devices = get_available_devices();

    devices
        .into_iter()
        .filter(|d| d.info.serial_number == Some(SERIAL_NUMBER.to_string()))
        .collect::<Vec<_>>()
}
