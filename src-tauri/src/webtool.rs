use serde_json::Value;

pub async fn duckduckgo_search(
    query: &str
) -> Result<String, String> {

    let query = urlencoding::encode(query);

    let url = format!(
        "https://api.duckduckgo.com/?q={}&format=json&no_html=1",
        query
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?;

    let json: Value = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let result = json["AbstractText"]
        .as_str()
        .unwrap_or("");
    
    if result.is_empty() {
        return Err("No results found".to_string());
    }

    Ok(result.to_string())
}