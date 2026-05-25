use std::process::Command;

pub fn get_battery_info() -> String {
    let output = Command::new("powershell")
        .args([
            "-Command",
            "$b=Get-CimInstance Win32_Battery; \
             Write-Output $b.EstimatedChargeRemaining; \
             Write-Output $b.BatteryStatus"
        ])
        .output();

    match output {
        Ok(output) => {
            let text = String::from_utf8_lossy(&output.stdout);

            let mut lines = text.lines();

            let battery = lines.next().unwrap_or("Unknown").trim();

            let status_code = lines.next().unwrap_or("0").trim();

            let status = match status_code {
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

            format!(
                "🔋 Battery: {}%\n⚡ Status: {}",
                battery,
                status
            )
        }
        Err(_) => {
            "Battery information unavailable".to_string()
        }
    }
}