// Markdown 渲染器 - 将解析结果转换为可视化内容
use super::parser::Parser;

pub struct Renderer;

impl Renderer {
    /// 将 Markdown 渲染为 HTML
    ///
    /// # 参数
    /// * `markdown` - Markdown 源文本
    ///
    /// # 返回
    /// 生成的 HTML 字符串
    pub fn render(markdown: &str) -> String {
        Parser::to_html(markdown)
    }

    /// 将 Markdown 渲染为纯文本预览（用于简单显示）
    ///
    /// # 参数
    /// * `markdown` - Markdown 源文本
    ///
    /// # 返回
    /// 格式化后的预览文本
    pub fn render_preview(markdown: &str) -> String {
        // 纯文本模式：保留原始文本，仅去掉行首的标题标记
        // 不处理 * 等行内格式，避免破坏内容
        markdown
            .lines()
            .map(|line| {
                // 仅移除行首的标题标记
                if line.starts_with('#') {
                    line.trim_start_matches('#').trim().to_string()
                } else {
                    line.to_string()
                }
            })
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_heading() {
        let markdown = "# Hello";
        let html = Renderer::render(markdown);
        assert!(html.contains("<h1>"));
    }

    #[test]
    fn test_render_preview_heading() {
        let markdown = "# Hello\n## World";
        let preview = Renderer::render_preview(markdown);
        assert_eq!(preview, "Hello\nWorld");
    }

    #[test]
    fn test_render_preview_preserves_italic() {
        // 验证纯文本预览保留 *italic* 格式
        let markdown = "*italic text*";
        let preview = Renderer::render_preview(markdown);
        assert_eq!(preview, "*italic text*");
    }

    #[test]
    fn test_render_preview_list() {
        // 列表项仍然显示完整文本（不再移除 - 标记）
        let markdown = "- item 1\n- item 2";
        let preview = Renderer::render_preview(markdown);
        assert_eq!(preview, "- item 1\n- item 2");
    }

    #[test]
    fn test_render_empty() {
        let markdown = "";
        let html = Renderer::render(markdown);
        assert_eq!(html, "");
    }
}
