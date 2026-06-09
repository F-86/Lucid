// Lucid - Typora 风格的 Markdown 编辑器
mod editor;
mod markdown;
mod ui;

use crate::editor::buffer::Buffer;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length};
use ui::Message;

// 应用状态
#[derive(Default)]
pub struct App {
    buffer: Buffer,
}

fn update(app: &mut App, message: Message) {
    match message {
        Message::EditInput(content) => {
            app.buffer.set_content(&content);
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

    // 预览面板
    let preview =
        scrollable(text(format!("预览\n{}", app.buffer.content()))).width(Length::FillPortion(1));

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
