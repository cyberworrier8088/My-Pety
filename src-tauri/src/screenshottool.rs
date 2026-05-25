use screenshots::Screen;

pub fn take_screenshot()
    -> Result<String, String>
{
    let screens =
        Screen::all()
            .map_err(|e| e.to_string())?;

    let screen =
        screens
            .first()
            .ok_or("No screen found")?;

    let image =
        screen
            .capture()
            .map_err(|e| e.to_string())?;

    let path = "screenshot.png";

    image
        .save(path)
        .map_err(|e| e.to_string())?;

    Ok(path.to_string())
}

use base64::{
    engine::general_purpose,
    Engine as _
};

pub fn image_to_base64(
    path: &str
) -> Result<String,String> {

    let bytes =
        std::fs::read(path)
            .map_err(|e| e.to_string())?;

    Ok(
        general_purpose::STANDARD
            .encode(bytes)
    )
}