use std::process::Command; // command module

pub fn set_brightness(level: u8) -> Result<String, String> { // set brightness function

    if level > 100 {
        return Err("Brightness must be 0-100".to_string()); // error if level is not 0-100
    }

    let script = format!("(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1,{})", level); // powershell script

    Command::new("powershell").args(["-Command", &script]).spawn().map_err(|e| e.to_string())?; // sexecute powershell script

    Ok(format!("Brightness set to {}%", level)) // return success message
}