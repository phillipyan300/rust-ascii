//! HTML generation module for converting ASCII art to web visualization

use anyhow::Result;

/// Configuration for HTML generation
#[derive(Debug, Clone)]
pub struct HtmlConfig {
    pub font_size: u32,
    pub background_color: String,
    pub text_color: String,
    pub font_family: String,
}

impl Default for HtmlConfig {
    fn default() -> Self {
        Self {
            font_size: 1,
            background_color: "000000".to_string(),
            text_color: "ffffff".to_string(),
            font_family: "monospace".to_string(),
        }
    }
}

/// Convert ASCII art text to HTML with the specified configuration
pub fn ascii_to_html(ascii_content: &str, config: HtmlConfig) -> Result<String> {
    let lines: Vec<&str> = ascii_content.lines().collect();
    if lines.is_empty() {
        return Ok(String::new());
    }
    
    let _max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let _height = lines.len();
    
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ASCII Art Visualization</title>
    <style>
        body {{
            margin: 0;
            padding: 20px;
            background-color: #{};
            color: #{};
            font-family: {}, monospace;
            font-size: {}px;
            line-height: 1;
            overflow: auto;
        }}
        
        .ascii-container {{
            white-space: pre;
            letter-spacing: 0;
            word-spacing: 0;
            display: inline-block;
        }}
        
        /* Responsive scaling */
        @media (max-width: 1200px) {{
            body {{ font-size: {}px; }}
        }}
        
        @media (max-width: 800px) {{
            body {{ font-size: {}px; }}
        }}
        
        @media (max-width: 600px) {{
            body {{ font-size: {}px; }}
        }}
    </style>
</head>
<body>
    <div class="ascii-container">{}</div>
    
    <script>
        let fontSize = {};
        const body = document.body;
        
        function updateFontSize() {{
            body.style.fontSize = fontSize + 'px';
        }}
        
        document.addEventListener('keydown', function(e) {{
            if (e.ctrlKey || e.metaKey) {{
                if (e.key === '+' || e.key === '=') {{
                    e.preventDefault();
                    fontSize = Math.min(fontSize + 1, 20);
                    updateFontSize();
                }} else if (e.key === '-') {{
                    e.preventDefault();
                    fontSize = Math.max(fontSize - 1, 1);
                    updateFontSize();
                }} else if (e.key === '0') {{
                    e.preventDefault();
                    fontSize = {};
                    updateFontSize();
                }}
            }}
        }});
        
        document.addEventListener('wheel', function(e) {{
            if (e.ctrlKey || e.metaKey) {{
                e.preventDefault();
                if (e.deltaY < 0) {{
                    fontSize = Math.min(fontSize + 1, 20);
                }} else {{
                    fontSize = Math.max(fontSize - 1, 1);
                }}
                updateFontSize();
            }}
        }});
    </script>
</body>
</html>"#,
        config.background_color,
        config.text_color,
        config.font_family,
        config.font_size,
        config.font_size.saturating_sub(1).max(1),
        config.font_size.saturating_sub(2).max(1),
        config.font_size.saturating_sub(3).max(1),
        html_escape::encode_text(ascii_content),
        config.font_size,
        config.font_size
    );
    
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_to_html() {
        let ascii = "Hello\nWorld";
        let config = HtmlConfig::default();
        let html = ascii_to_html(ascii, config).unwrap();
        
        assert!(html.contains("Hello"));
        assert!(html.contains("World"));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_empty_ascii() {
        let config = HtmlConfig::default();
        let html = ascii_to_html("", config).unwrap();
        assert_eq!(html, "");
    }
}
