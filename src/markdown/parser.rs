// Markdown 解析器 - 使用 pulldown-cmark

/// Markdown 解析结果
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParsedMarkdown {
    /// 解析后的 HTML
    pub html: String,
    /// 原始 Markdown 内容
    pub source: String,
}

pub struct Parser;

impl Parser {
    /// 将 Markdown 文本解析为 HTML
    ///
    /// # 参数
    /// * `markdown` - Markdown 源文本
    ///
    /// # 返回
    /// 解析结果，包含生成的 HTML 和原始内容
    pub fn parse(markdown: &str) -> Result<ParsedMarkdown, String> {
        // 使用 pulldown-cmark 解析 Markdown
        let parser = pulldown_cmark::Parser::new(markdown);

        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        Ok(ParsedMarkdown {
            html,
            source: markdown.to_string(),
        })
    }

    /// 仅获取 HTML，忽略源文本
    pub fn to_html(markdown: &str) -> String {
        match Self::parse(markdown) {
            Ok(parsed) => parsed.html,
            Err(_) => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let markdown = "# Hello World";
        let result = Parser::parse(markdown).unwrap();
        assert!(result.html.contains("<h1>"));
        assert!(result.html.contains("Hello World"));
    }

    #[test]
    fn test_parse_bold() {
        let markdown = "**bold text**";
        let html = Parser::to_html(markdown);
        assert!(html.contains("<strong>"));
        assert!(html.contains("bold text"));
    }

    #[test]
    fn test_parse_list() {
        let markdown = "- item 1\n- item 2";
        let html = Parser::to_html(markdown);
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>"));
    }

    #[test]
    fn test_parse_heading_without_space() {
        // Markdown 要求标题后必须有空格
        // ##sdad 会被当作普通段落文本
        let markdown = "##sdad";
        let html = Parser::to_html(markdown);
        println!("'##sdad' 解析结果: {}", html);
        // 这不会生成 <h2> 因为没有空格
        assert!(html.contains("##sdad"));
    }

    #[test]
    fn test_parse_heading_with_space() {
        // 正确的标题格式：## 后面必须有空格
        let markdown = "## sdad";
        let html = Parser::to_html(markdown);
        println!("'## sdad' 解析结果: {}", html);
        assert!(html.contains("<h2>"));
        assert!(html.contains("sdad"));
    }
}
