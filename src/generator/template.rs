pub struct TemplateEngine;

impl TemplateEngine {
    /// Wraps raw compiled body HTML into a full HTML5 boilerplate template
    pub fn wrap_to_full_page(title: &str, body_content: &str) -> String {
        let template = concat!(
            "<!DOCTYPE html>\n",
            "<html lang=\"en\">\n",
            "<head>\n",
            "    <meta charset=\"UTF-8\">\n",
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
            "    <title>{title}</title>\n",
            "    <style>\n",
            "        body { font-family: sans-serif; line-height: 1.6; max-width: 800px; margin: 40px auto; padding: 0 20px; color: #333; }\n",
            "        pre { background: #f4f4f4; padding: 15px; border-radius: 5px; overflow-x: auto; }\n",
            "        code { font-family: monospace; background: #f4f4f4; padding: 2px 4px; border-radius: 3px; }\n",
            "        pre code { padding: 0; background: none; }\n",
            "        .tim-note { border-left: 4px solid #0076ff; background: #f0f7ff; padding: 15px; margin: 20px 0; border-radius: 0 5px 5px 0; }\n",
            "    </style>\n",
            "</head>\n",
            "<body>\n",
            "    {body_content}\n",
            "</body>\n",
            "</html>"
        );

        // A quick, simple replace to hydrate our static template
        template
            .replace("{title}", title)
            .replace("{body_content}", body_content)
    }
}
