use windows::{
    core::Result,
    Win32::{
        Foundation::{ERROR_INSUFFICIENT_BUFFER, MAX_PATH, PROPERTYKEY},
        Media::Audio::IMMDevice,
        System::Com::{
            StructuredStorage::{PropVariantToString, PROPVARIANT},
            STGM_READ,
        },
    },
};

pub fn get_pkey_value(device: &IMMDevice, pkey_value: &PROPERTYKEY) -> Result<String> {
    let read_buffer = unsafe {
        let props = device.OpenPropertyStore(STGM_READ)?;
        let name_prop: PROPVARIANT = props.GetValue(pkey_value)?;

        // growable buffer if needed.
        let mut buffer_size = MAX_PATH;
        let max_buffer_limit = MAX_PATH * 126; // 260 * 128 ~ 32kb
        loop {
            let mut buffer = vec![0u16; buffer_size as usize];

            match PropVariantToString(&name_prop, &mut buffer) {
                Ok(_) => break Ok(buffer),
                Err(e) if e.code() == ERROR_INSUFFICIENT_BUFFER.into() => {
                    buffer_size *= 2;
                    if buffer_size > max_buffer_limit {
                        break Err(e);
                    }
                }
                Err(e) => break Err(e),
            }
        }
    }?;

    Ok(super::process_lossy_name(&read_buffer))
}
