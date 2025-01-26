use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Error;
use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

/// Fetch and parse sitemaps recursively, saving discovered URLs to a file.
pub async fn fetch_and_parse_sitemap(
    main_sitemap_url: &str,
    output_file: &str,
) -> Result<(), Error> {

    let mut all_urls = Vec::new();
    let mut sitemap_queue = VecDeque::new();

    sitemap_queue.push_back(main_sitemap_url.to_string());

    // Start processing the sitemap URLs in the queue
    while let Some(sitemap_url) = sitemap_queue.pop_front() {
        match reqwest::get(&sitemap_url).await {
            Ok(response) => match response.text().await {
                Ok(content) => {
                    // Parse the sitemap and extract URLs
                    let urls = parse_sitemap(&content);
                    // Process each URL found
                    for url in urls {
                        if url.ends_with(".xml") {
                            sitemap_queue.push_back(url); // Add sitemap URLs to queue
                        } else {
                            all_urls.push(url); // Add regular URLs to the list
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to read content from {}: {:?}", sitemap_url, err);
                }
            },
            Err(err) => {
                eprintln!("Failed to fetch {}: {:?}", sitemap_url, err);
            }
        }
    }

    // Save all discovered URLs to a file
    save_urls_to_file(output_file, &all_urls);
    Ok(())
}

/// Parse the sitemap XML content and extract all URLs from <loc> elements.
fn parse_sitemap(content: &str) -> Vec<String> {
    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut urls = Vec::new();

    // Process XML events to extract URLs
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == quick_xml::name::QName(b"loc") => {

                // Extract the URL from the <loc> element
                match reader.read_text(e.name()) {
                    Ok(url) => {
                        let url_string = url.to_string();
                        urls.push(url_string);
                    }
                    Err(err) => {
                        eprintln!("Failed to parse <loc>: {:?}", err);
                    }
                }
            }
            Ok(Event::Eof) => break, // End of file
            Err(err) => {
                eprintln!("Error parsing XML: {:?}", err);
                break;
            }
            _ => (),
        }
        buf.clear();
    }

    urls
}

/// Save the list of URLs to a file.
fn save_urls_to_file(file_path: &str, urls: &[String]) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to open file");

    let mut writer = BufWriter::new(file);

    // Write each URL to the file and print status
    for url in urls {
        if let Err(err) = writeln!(writer, "{}", url) {
            eprintln!("Failed to write URL to file: {:?}", err);
        } else {
            println!("Saved URL: {}", url);
        }
    }
}
