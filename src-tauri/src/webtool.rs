use regex::Regex;
use serde_json::Value;

struct SearchResult {
    title: String,
    url: String,
    snippet: String,
}

pub async fn duckduckgo_search(query: &str) -> Result<String, String> {
    let cleaned_query = clean_query(query);
    if cleaned_query.is_empty() {
        return Err("Please provide a more specific search query.".to_string());
    }

    let instant_answer = fetch_instant_answer(&cleaned_query).await.ok();
    let mut results = fetch_html_results(&cleaned_query).await.unwrap_or_default();

    if results.is_empty() {
        if let Some(answer) = instant_answer {
            return Ok(format!("Quick answer:\n{}", answer));
        }

        return Err("No strong web results found. Try a more specific query.".to_string());
    }

    prioritize_official_results(&mut results);
    dedupe_results(&mut results);
    results.truncate(3);

    let mut output = Vec::new();

    if let Some(answer) = instant_answer {
        output.push(format!("Quick answer:\n{}", answer));
    }

    output.push("Top web results:".to_string());

    for (index, result) in results.iter().enumerate() {
        output.push(format!(
            "{}. {}\n{}\n{}",
            index + 1,
            result.title,
            result.snippet,
            result.url
        ));
    }

    Ok(output.join("\n\n"))
}

fn clean_query(query: &str) -> String {
    let cleaned = query
        .replace("search", "")
        .replace("Search", "")
        .replace("find", "")
        .replace("Find", "")
        .replace("look up", "")
        .replace("Look up", "")
        .replace("google", "")
        .replace("Google", "")
        .replace("web", "")
        .replace("Web", "");

    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}

async fn fetch_instant_answer(query: &str) -> Result<String, String> {
    let encoded = urlencoding::encode(query);
    let url = format!(
        "https://api.duckduckgo.com/?q={}&format=json&no_html=1",
        encoded
    );

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let json: Value = response.json().await.map_err(|e| e.to_string())?;

    let answer = json["AbstractText"].as_str().unwrap_or("").trim();

    if answer.is_empty() {
        return Err("No instant answer".to_string());
    }

    Ok(answer.to_string())
}

async fn fetch_html_results(query: &str) -> Result<Vec<SearchResult>, String> {
    let encoded = urlencoding::encode(query);
    let url = format!("https://html.duckduckgo.com/html/?q={}", encoded);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| e.to_string())?;

    let html = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    parse_html_results(&html)
}

fn parse_html_results(html: &str) -> Result<Vec<SearchResult>, String> {
    let result_re = Regex::new(
        r#"<a[^>]*class="result__a"[^>]*href="(?P<url>[^"]+)"[^>]*>(?P<title>.*?)</a>[\s\S]*?<a[^>]*class="result__snippet"[^>]*>(?P<snippet>.*?)</a>"#
    )
    .map_err(|e| e.to_string())?;

    let tag_re = Regex::new(r"<[^>]+>").map_err(|e| e.to_string())?;

    let mut results = Vec::new();

    for caps in result_re.captures_iter(html) {
        let raw_title = caps.name("title").map(|m| m.as_str()).unwrap_or("");
        let raw_url = caps.name("url").map(|m| m.as_str()).unwrap_or("");
        let raw_snippet = caps.name("snippet").map(|m| m.as_str()).unwrap_or("");

        let title = decode_html(&tag_re.replace_all(raw_title, "").trim());
        let snippet = decode_html(&tag_re.replace_all(raw_snippet, "").trim());
        let url = decode_duckduckgo_url(raw_url);

        if title.is_empty() || url.is_empty() {
            continue;
        }

        results.push(SearchResult { title, url, snippet });
    }

    Ok(results)
}

fn decode_duckduckgo_url(raw_url: &str) -> String {
    let decoded = raw_url.replace("&amp;", "&");

    if let Some(start) = decoded.find("uddg=") {
        let value = &decoded[start + 5..];
        let end = value.find('&').unwrap_or(value.len());
        let encoded_target = &value[..end];

        return urlencoding::decode(encoded_target)
            .map(|v| v.into_owned())
            .unwrap_or_else(|_| encoded_target.to_string());
    }

    decoded
}

fn decode_html(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
}

fn prioritize_official_results(results: &mut [SearchResult]) {
    results.sort_by_key(|result| {
        let url = result.url.to_lowercase();
        let official = url.contains(".gov")
            || url.contains(".edu")
            || url.contains("wikipedia.org")
            || url.contains("docs.")
            || url.contains("developer.");

        if official { 0 } else { 1 }
    });
}

fn dedupe_results(results: &mut Vec<SearchResult>) {
    let mut seen = std::collections::HashSet::new();
    results.retain(|result| seen.insert(result.url.clone()));
}
