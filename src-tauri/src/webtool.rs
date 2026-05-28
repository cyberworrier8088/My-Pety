use regex::Regex; // importing regex lib for pattern matching
use serde_json::Value; // importing serde json for json serialization

struct SearchResult {title: String, url: String, snippet: String} // struct for search result

pub async fn duckduckgo_search(query: &str) -> Result<String, String> { // function to search on duckduckgo
    let cleaned_query = clean_query(query); // cleaning qury
    if cleaned_query.is_empty() {
        return Err("Please provide a more specific search query.".to_string());
    }

    let instant_answer = fetch_instant_answer(&cleaned_query).await.ok(); // fetching instant answer
    let mut results = fetch_html_results(&cleaned_query).await.unwrap_or_default(); // fetching html results

    if results.is_empty() { 
        if let Some(answer) = instant_answer { // if instant answer is found
            return Ok(format!("Quick answer:\n{}", answer)); // returning instant answer
        }

        return Err("No strong web results found. Try a more specific query.".to_string()); // returning error
    }

    prioritize_official_results(&mut results); // prioritizing official results
    dedupe_results(&mut results); // deduplicating results
    results.truncate(3); // truncating results to 3

    let mut output = Vec::new(); // creating output vector

    if let Some(answer) = instant_answer { // if instant answer is found
        output.push(format!("Quick answer:\n{}", answer)); // adding instant answer to output
    }

    output.push("Top web results:".to_string()); // top web results header

    for (index, result) in results.iter().enumerate() { // iterating through results
        output.push(format!("{}. {}\n{}\n{}", index + 1, result.title, result.snippet, result.url)); // adding result to output
    }

    Ok(output.join("\n\n")) // returning output
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

    cleaned.split_whitespace().collect::<Vec<_>>().join(" ") // joining cleaned query
}

async fn fetch_instant_answer(query: &str) -> Result<String, String> { // fetching instant answer
    
    let encoded = urlencoding::encode(query); // encoding query
    
    let url = format!("https://api.duckduckgo.com/?q={}&format=json&no_html=1", encoded); // creating url

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?; // getting response
    
    let json: Value = response.json().await.map_err(|e| e.to_string())?; // parsing json

    let answer = json["AbstractText"].as_str().unwrap_or("").trim(); // getting answer

    if answer.is_empty() {
        return Err("No instant answer".to_string()); // returning error if no answer
    }

    Ok(answer.to_string()) // returning answer
}

async fn fetch_html_results(query: &str) -> Result<Vec<SearchResult>, String> { // fetching html results
    let encoded = urlencoding::encode(query); // encoding query
    let url = format!("https://html.duckduckgo.com/html/?q={}", encoded); // creating url

    let client = reqwest::Client::builder().user_agent("Mozilla/5.0").build().map_err(|e| e.to_string())?;

    let html = client.get(url).send().await.map_err(|e| e.to_string())?.text().await.map_err(|e| e.to_string())?; // getting html

    parse_html_results(&html)
}


//this for parse lofgic
fn parse_html_results(html: &str) -> Result<Vec<SearchResult>, String> { // parsing html results
    let result_re = Regex::new(
        r#"<a[^>]*class="result__a"[^>]*href="(?P<url>[^"]+)"[^>]*>(?P<title>.*?)</a>[\s\S]*?<a[^>]*class="result__snippet"[^>]*>(?P<snippet>.*?)</a>"#
    ).map_err(|e| e.to_string())?; // compiling regex

    let tag_re = Regex::new(r"<[^>]+>").map_err(|e| e.to_string())?; // compiling regex

    let mut results = Vec::new(); // vec new

    for caps in result_re.captures_iter(html) { // iterating through captures
        let raw_title = caps.name("title").map(|m| m.as_str()).unwrap_or(""); // getting title
        let raw_url = caps.name("url").map(|m| m.as_str()).unwrap_or(""); // getting url
        let raw_snippet = caps.name("snippet").map(|m| m.as_str()).unwrap_or(""); // getting snippet

        let title = decode_html(&tag_re.replace_all(raw_title, "").trim()); // decoding html
        let snippet = decode_html(&tag_re.replace_all(raw_snippet, "").trim()); // decoding html
        let url = decode_duckduckgo_url(raw_url); // decoding duckduckgo url

        if title.is_empty() || url.is_empty() { // checking if title and url is empty
            continue;
        }

        results.push(SearchResult { title, url, snippet }); // pushing result
    }

    Ok(results)
}

fn decode_duckduckgo_url(raw_url: &str) -> String { // decoding duckduckgo url
    let decoded = raw_url.replace("&amp;", "&"); // replacing &amp; with &

    if let Some(start) = decoded.find("uddg=") { // finding uddg
        let value = &decoded[start + 5..]; // getting value
        let end = value.find('&').unwrap_or(value.len()); // getting end
        let encoded_target = &value[..end]; // getting encoded target

        return urlencoding::decode(encoded_target)
            .map(|v| v.into_owned())
            .unwrap_or_else(|_| encoded_target.to_string());
    }

    decoded
}

fn decode_html(input: &str) -> String { // decoding html
    input
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
}

fn prioritize_official_results(results: &mut [SearchResult]) { // prioritizing official results
    results.sort_by_key(|result| {
        let url = result.url.to_lowercase();
        let official = url.contains(".gov")
            || url.contains(".edu")
            || url.contains("wikipedia.org")
            || url.contains("docs.")
            || url.contains("developer.");

        if official { 0 } else { 1 } // if official is true, return 0, otherwise return 1
    });
}

fn dedupe_results(results: &mut Vec<SearchResult>) { // deduplicating results
    let mut seen = std::collections::HashSet::new(); // creating hash set
    results.retain(|result| seen.insert(result.url.clone())); // retaining results
}
