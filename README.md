# XML Collected Library

A Rust library to fetch and parse XML sitemaps from URLs, discover sub-sitemaps, and store the extracted URLs into a
specified file.

## Features

- Fetches sitemap XML documents from a URL.
- Parses the sitemap and extracts all URLs from `<loc>` tags.
- Recursively discovers and processes other sitemaps linked in `<loc>` tags.
- Saves all discovered URLs into a specified file.
- Handles both simple and nested XML sitemap structures.

## Installation

To use this library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
quick-xml = "0.37.2"
reqwest = { version = "0.12", features = ["blocking", "json"] }


```

## Usage

Below is an example of how to use the library in your project.
   
```rust 
    use xml_collected::fetch_and_parse_sitemap;
    
    #[tokio::main]
    async fn main() {
    let sitemap_url = "https://example.com/sitemap.xml";
    let output_path = "urls.txt";
    
        // Fetch and parse the sitemap, saving URLs to a file
        if let Err(err) = fetch_and_parse_sitemap(sitemap_url, output_path).await {
            eprintln!("Error: {}", err);
        } else {
            println!("URLs have been saved to {}", output_path);
        }
    }
```
## Functions

### fetch_and_parse_sitemap

This is the main function of the library. It fetches the sitemap XML from a given URL, parses it, and stores all
discovered URLs in a specified file.
Arguments:

    sitemap_url: A string containing the URL to the sitemap XML.
    output_file_path: A string containing the path where the URLs should be saved.

Return:

    Returns a Result<(), Box<dyn std::error::Error>>, where Ok(()) means the operation was successful, and Err contains an error message if something goes wrong.

### Example:
```rust 
    async fn fetch_and_parse_sitemap(sitemap_url: &str, output_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Your function implementation here
    }
```
This function will fetch the sitemap from the given URL, recursively parse all URLs (including sub-sitemaps), and store
the URLs in the provided output file.
How It Works:

- The function fetches the sitemap from the provided URL.
- It parses the sitemap's XML content using the quick_xml library.
- It extracts URLs inside <loc> tags.
- If the URL points to another sitemap, it will add it to the queue for further parsing.
- If the URL is a regular webpage, it will save it to the output file.
- The process repeats until all sitemaps have been processed.

### Error Handling

If any error occurs during fetching or parsing, the function will print an error message to the standard error.
Contributing

If you find bugs or have ideas for improvements, feel free to open an issue or submit a pull request. Contributions are
welcome!


### License

This project is licensed under the MIT License - see the LICENSE file for details.