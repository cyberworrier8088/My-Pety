use std::process::Command;

pub fn set_brightness(
    level: u8
) -> Result<String, String> {

    if level > 100 {
        return Err(
            "Brightness must be 0-100"
                .to_string()
        );
    }

    let script = format!(
        "(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1,{})",
        level
    );

    Command::new("powershell")
        .args([
            "-Command",
            &script
        ])
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(format!(
        "Brightness set to {}%",
        level
    ))
}