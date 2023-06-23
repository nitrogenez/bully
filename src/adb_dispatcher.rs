use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;

use mozdevice::{Device, DeviceInfo, Host};

use tracing::{error, info};

use crate::PasswdGenerator;
use crate::PinGenerator;

#[derive(Debug)]
pub struct Dispatcher {
    pub device: Device,
}

impl Dispatcher {
    pub fn new() -> Result<Dispatcher> {
        info!("Connecting to adb server...");

        let server = Host {
            ..Default::default()
        };
        info!("Searching for connected devices...");

        let devices: Vec<DeviceInfo> = server.devices().context("Failed to connect to adb")?;

        if devices.is_empty() {
            error!("No devices found");
            std::process::exit(1);
        }
        info!("Connected devices: {}", devices.len());

        let active_devinfo = devices.get(0).unwrap();
        let active_device = Device::new(
            server,
            active_devinfo.serial.clone(),
            mozdevice::AndroidStorageInput::Auto,
        )
        .context("Failed to get active device")?;

        info!(
            "Connected to device: {:?} ({:?})",
            active_devinfo.info["device"], active_devinfo.info["model"]
        );

        Ok(Dispatcher {
            device: active_device,
        })
    }

    pub fn wake_device(&self) -> Result<()> {
        self.device
            .execute_host_shell_command("input keyevent 82")
            .context("Failed to run shell command")?;
        self.device
            .execute_host_shell_command("input swipe 408 1210 508 85")
            .context("Failed to run shell command")?;
        std::thread::sleep(std::time::Duration::from_millis(500));
        Ok(())
    }

    pub fn push_pin(&self, generator: &PinGenerator) -> Result<()> {
        for i in generator.adb_keycodes.iter() {
            self.device
                .execute_host_shell_command(&format!("input keyevent {:?}", i))
                .context("Failed to push combination")?;
        }

        self.device
            .execute_host_shell_command("input keyevent 66")
            .context("Failed to push combination")?;

        Ok(())
    }

    pub fn push_passwd(&self, generator: &PasswdGenerator) -> Result<()> {
        self.device
            .execute_host_shell_command(&format!("input text {:?}", generator.passwd_raw))
            .context("Failed to push password")?;
        self.device
            .execute_host_shell_command("input keyevent 66")
            .context("Failed to push password")?;

        Ok(())
    }

    pub fn push(&self, data: &str) -> Result<()> {
        self.device
            .execute_host_shell_command(&format!("input text {:?}", data))
            .context(format!("Failed to push {:?}", data))?;
        Ok(())
    }

    pub fn send_enter(&self) -> Result<()> {
        self.device
            .execute_host_shell_command("input keyevent 66")
            .context("Failed to send Enter key to the device")?;
        Ok(())
    }
}
