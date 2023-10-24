extern crate frontmatter;

use tera::Tera;
use pulldown_cmark::{Parser, Options, html};
use frontmatter::{parse, parse_and_find_content, Yaml, ScanError};
use std::fs;
use std::io::{self, Read, Write};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    // Initialize Tera and add your templates.
    let mut tera = Tera::default();
    tera.add_raw_template("base.html", include_str!("templates/base.html")).unwrap();

    // Iterate through the content directory.
    for entry in WalkDir::new("content").into_iter().filter_map(|entry| entry.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let markdown_with_front_matter = fs::read_to_string(&file_path)?;

            // Parse front matter using "frontmatter" crate.
            let front_matter_result: Result<Option<Yaml>, ScanError> = parse(&markdown_with_front_matter);
            
            match front_matter_result {
                Ok(front_matter_option) => {
                    // Parse the content (Markdown) into HTML.
                    let mut html_output = String::new();
                    let parser = Parser::new_ext(markdown_with_front_matter, Options::all());
                    html::push_html(&mut html_output, parser);

                    // Render the HTML with the base template.
                    let mut context = tera::Context::new();
                    context.insert("markdown_content", &html_output);
                    
                    if let Some(front_matter) = front_matter_option {
                        // Access front matter values, e.g., title.
                        context.insert("title", &front_matter);
                    }

                    let rendered_content = tera.render("base.html", &context)?;

                    // Define the output file path (in the output directory).
                    let output_file_name = format!(
                        "output/{}",
                        file_path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("output.html")
                    );

                    // Write the rendered HTML to the output file.
                    let mut output_file = fs::File::create(output_file_name)?;

                    output_file.write_all(rendered_content.as_bytes())?;

                    println!("Markdown file '{}' rendered to HTML: {:?}", file_path.display(), output_file_name);
                }
                Err(err) => {
                    println!("Error parsing front matter: {:?}", err);
                }
            }
        }
    }

    Ok(())
}

