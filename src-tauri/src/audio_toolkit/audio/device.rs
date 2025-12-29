use cpal::traits::{DeviceTrait, HostTrait};

pub struct CpalDeviceInfo {
    pub index: String,
    pub name: String,
    pub is_default: bool,
    pub device: cpal::Device,
}

/// Check if a device can provide a valid input configuration
fn has_valid_input_config(device: &cpal::Device) -> bool {
    // Check if device has any supported input configs
    if let Ok(mut configs) = device.supported_input_configs() {
        if configs.next().is_some() {
            // Also verify we can get a default config
            return device.default_input_config().is_ok();
        }
    }
    false
}

pub fn list_input_devices() -> Result<Vec<CpalDeviceInfo>, Box<dyn std::error::Error>> {
    let host = crate::audio_toolkit::get_cpal_host();
    let default_name = host.default_input_device().and_then(|d| d.name().ok());

    let mut out = Vec::<CpalDeviceInfo>::new();
    let mut valid_index = 0;

    for device in host.input_devices()? {
        // Skip devices that can't provide valid input configs
        if !has_valid_input_config(&device) {
            log::debug!(
                "Skipping device {:?}: no valid input configuration",
                device.name().unwrap_or_else(|_| "Unknown".into())
            );
            continue;
        }

        let name = device.name().unwrap_or_else(|_| "Unknown".into());
        let is_default = Some(name.clone()) == default_name;

        out.push(CpalDeviceInfo {
            index: valid_index.to_string(),
            name,
            is_default,
            device,
        });
        valid_index += 1;
    }

    Ok(out)
}

pub fn list_output_devices() -> Result<Vec<CpalDeviceInfo>, Box<dyn std::error::Error>> {
    let host = crate::audio_toolkit::get_cpal_host();
    let default_name = host.default_output_device().and_then(|d| d.name().ok());

    let mut out = Vec::<CpalDeviceInfo>::new();

    for (index, device) in host.output_devices()?.enumerate() {
        let name = device.name().unwrap_or_else(|_| "Unknown".into());

        let is_default = Some(name.clone()) == default_name;

        out.push(CpalDeviceInfo {
            index: index.to_string(),
            name,
            is_default,
            device,
        });
    }

    Ok(out)
}
