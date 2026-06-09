// Lucid - Typora 风格的 Markdown 编辑器
mod editor;
mod markdown;
mod ui;

use crate::editor::buffer::Buffer;
use crate::markdown::Renderer;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length};
use ui::Message;

// 应用状态
#[derive(Default)]
pub struct App {
    buffer: Buffer,
    /// 缓存的预览 HTML
    preview_html: String,
}

fn update(app: &mut App, message: Message) {
    match message {
        Message::EditInput(content) => {
            app.buffer.set_content(&content);
            // 实时更新预览 HTML
            app.preview_html = Renderer::render(&content);
        }
        Message::FileOpen => {
            // TODO: 实现文件打开
        }
        Message::FileSave => {
            // TODO: 实现文件保存
        }
    }
}

fn view(app: &App) -> Element<'_, Message> {
    // 工具栏
    let toolbar = row![
        button("📁 Open").on_press(Message::FileOpen).padding(10),
        button("💾 Save").on_press(Message::FileSave).padding(10),
    ]
    .spacing(10);

    // 编辑器 - 文本输入框
    let editor = text_input("输入 Markdown...", app.buffer.content())
        .on_input(Message::EditInput)
        .padding(10)
        .width(Length::FillPortion(1));

    // 预览面板 - 使用 HTML 渲染（未来支持）
    // 当前 iced 不原生支持 HTML，所以显示原始 HTML 标签帮助用户理解
    let preview_html = Renderer::render(app.buffer.content());
    // 限制显示长度，避免过长的 HTML 被截断
    let display_text = if preview_html.is_empty() {
        "预览将在此显示".to_string()
    } else {
        preview_html.clone()
    };
    let preview = scrollable(text(display_text).width(Length::Fill))
        .width(Length::FillPortion(1))
        .height(Length::Fill);

    // 左右两栏布局
    let content = row![editor, preview]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill);

    // 组装应用
    let app_layout = column![toolbar, content]
        .spacing(10)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill);

    container(app_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn main() -> iced::Result {
    iced::run(update, view)
}
