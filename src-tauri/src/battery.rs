use std::process::Command; // importing command module

pub fn get_battery_info() -> String { // getting battery info
    let output = Command::new("powershell").args([ // using powershell command
            "-Command",
            "$b=Get-CimInstance Win32_Battery; \
             Write-Output $b.EstimatedChargeRemaining; \
             Write-Output $b.BatteryStatus"
        ]) // executing command
        .output(); // getting output

    match output {
        Ok(output) => {
            let text = String::from_utf8_lossy(&output.stdout); // getting output as string

            let mut lines = text.lines(); // splitting output into lines

            let battery = lines.next().unwrap_or("Unknown").trim(); // getting battery percentage

            let status_code = lines.next().unwrap_or("0").trim(); // getting battery status code

            let status = match status_code { // matching battery status code
                "1" => "Discharging",
                "2" => "Charging",
                "3" => "Fully Charged",
                "4" => "Low",
                "5" => "Critical",
                "6" => "Charging",
                "7" => "Charging and High",
                "8" => "Charging and Low",
                "9" => "Charging and Critical",
                "11" => "Partially Charged",
                _ => "Unknown",
            };

            format!("🔋 Battery: {}%\n⚡ Status: {}", battery, status) // formatting battery info
        }
        Err(_) => {
            "Battery information unavailable".to_string() // returning error message
        }
    }
}