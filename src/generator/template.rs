pub struct TemplateEngine;

impl TemplateEngine {
    /// Wraps raw compiled body HTML into a full HTML5 boilerplate template
    pub fn wrap_to_full_page(title: &str, body_content: &str) -> String {
        let template = concat!(
            "<!DOCTYPE html>",
            "<html lang=\"en\">",
            "<head>",
            "    <meta charset=\"UTF-8\">",
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">",
            "    <title>{title}</title>",
            "    <style>",
            "        body { font-family: sans-serif; line-height: 1.6; max-width: 800px; margin: 40px auto; padding: 0 20px; color: #333; }",
            "        pre { background: #f4f4f4; padding: 15px; border-radius: 5px; overflow-x: auto; }",
            "        code { font-family: monospace; background: #f4f4f4; padding: 2px 4px; border-radius: 3px; }",
            "        pre code { padding: 0; background: none; }",
            "        .tim-note { border-left: 4px solid #0076ff; background: #f0f7ff; padding: 15px; margin: 20px 0; border-radius: 0 5px 5px 0; }",
            "    </style>",
            "</head>",
            "<body>",
            "    {body_content}",
            "</body>",
            "</html>"
        );

        // A quick, simple replace to hydrate our static template
        template
            .replace("{title}", title)
            .replace("{body_content}", body_content)
    }
}
