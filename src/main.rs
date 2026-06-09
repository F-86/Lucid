// Lucid - Typora 风格的 Markdown 编辑器
mod editor;
mod markdown;
mod ui;

use crate::editor::buffer::Buffer;
use crate::editor::history::History;
use crate::markdown::Renderer;
use iced::widget::{button, column, container, row, scrollable, text, text_editor};
use iced::{Element, Length, Task, alignment};
use std::path::PathBuf;
use ui::Message;

/// 应用状态结构
#[derive(Default)]
pub struct App {
    buffer: Buffer,
    /// 编辑器内容状态
    editor_content: text_editor::Content,
    /// 缓存的预览 HTML
    preview_html: String,
    /// 当前打开的文件路径
    file_path: Option<PathBuf>,
    /// 是否有未保存修改
    is_modified: bool,
    /// 状态栏消息
    status_message: String,
    /// 撤销/重做历史栈
    history: History,
    /// 撤销按钮是否启用
    can_undo: bool,
    /// 重做按钮是否启用
    can_redo: bool,
}

/// 处理消息并返回可能的异步任务
fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::EditInput(content) => {
            app.buffer.set_content(&content);
            // 实时更新预览 HTML
            app.preview_html = Renderer::render(&content);
            app.is_modified = true;
            // 记录编辑历史
            app.history.push(content);
            // 更新按钮状态
            app.can_undo = app.history.can_undo();
            app.can_redo = app.history.can_redo();
            Task::none()
        }

        Message::EditorAction(action) => {
            app.editor_content.perform(action);
            let content = app.editor_content.text();
            app.buffer.set_content(&content);
            app.preview_html = Renderer::render(&content);
            app.is_modified = true;
            // 记录编辑历史
            app.history.push(content.to_string());
            // 更新按钮状态
            app.can_undo = app.history.can_undo();
            app.can_redo = app.history.can_redo();
            Task::none()
        }

        Message::FileOpen => {
            // 打开文件对话框 + 异步读取
            Task::perform(
                async {
                    let handle = rfd::AsyncFileDialog::new()
                        .add_filter("Markdown", &["md", "txt"])
                        .add_filter("所有文件", &["*"])
                        .set_title("打开文件")
                        .pick_file()
                        .await;

                    match handle {
                        None => Err("已取消".to_string()),
                        Some(h) => {
                            let path = h.path().to_path_buf();
                            match editor::file::read_file(&path).await {
                                Ok(content) => Ok((path, content)),
                                Err(e) => Err(format!("读取失败：{}", e)),
                            }
                        }
                    }
                },
                Message::FileOpened,
            )
        }

        Message::FileSave => {
            if let Some(path) = &app.file_path {
                // 已有路径，直接保存
                let path = path.clone();
                let content = app.buffer.content().to_string();
                Task::perform(
                    async move {
                        match editor::file::write_file(&path, &content).await {
                            Ok(()) => Ok(()),
                            Err(e) => Err(format!("保存失败：{}", e)),
                        }
                    },
                    Message::FileSaved,
                )
            } else {
                // 无路径，弹出另存为对话框
                let content = app.buffer.content().to_string();
                Task::perform(
                    async move {
                        let handle = rfd::AsyncFileDialog::new()
                            .add_filter("Markdown", &["md"])
                            .add_filter("文本文件", &["txt"])
                            .set_title("保存文件")
                            .save_file()
                            .await;

                        match handle {
                            None => Err("已取消".to_string()),
                            Some(h) => {
                                let path = h.path().to_path_buf();
                                match editor::file::write_file(&path, &content).await {
                                    Ok(()) => Ok(()),
                                    Err(e) => Err(format!("保存失败：{}", e)),
                                }
                            }
                        }
                    },
                    Message::FileSaved,
                )
            }
        }

        Message::FileOpened(Ok((path, content))) => {
            app.buffer.set_content(&content);
            app.editor_content = text_editor::Content::with_text(&content);
            app.preview_html = Renderer::render(&content);
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("未知文件")
                .to_string();
            app.file_path = Some(path);
            app.is_modified = false;
            app.status_message = format!("✓ 已打开：{name}");
            // 更新按钮状态
            app.can_undo = app.history.can_undo();
            app.can_redo = app.history.can_redo();
            Task::none()
        }

        Message::FileOpened(Err(e)) => {
            if e != "已取消" {
                app.status_message = format!("✗ {e}");
            }
            Task::none()
        }

        Message::FileSaved(Ok(())) => {
            app.is_modified = false;
            app.status_message = "✓ 已保存".to_string();
            Task::none()
        }

        Message::FileSaved(Err(e)) => {
            if e != "已取消" {
                app.status_message = format!("✗ {e}");
            }
            Task::none()
        }

        Message::Undo => {
            // 撤销：从撤销栈弹出前一个状态
            if let Some(previous) = app.history.undo() {
                app.editor_content = text_editor::Content::with_text(&previous);
                app.buffer.set_content(&previous);
                app.preview_html = Renderer::render(&previous);
                app.is_modified = true;
                app.status_message = "↶ 已撤销".to_string();
                app.can_undo = app.history.can_undo();
                app.can_redo = app.history.can_redo();
            } else {
                app.status_message = "⚠ 没有可撤销的操作".to_string();
            }
            Task::none()
        }

        Message::Redo => {
            // 重做：从重做栈弹出下一个状态
            if let Some(next) = app.history.redo() {
                app.editor_content = text_editor::Content::with_text(&next);
                app.buffer.set_content(&next);
                app.preview_html = Renderer::render(&next);
                app.is_modified = true;
                app.status_message = "↷ 已重做".to_string();
                app.can_undo = app.history.can_undo();
                app.can_redo = app.history.can_redo();
            } else {
                app.status_message = "⚠ 没有可重做的操作".to_string();
            }
            Task::none()
        }
    }
}

/// 构建 UI
fn view(app: &App) -> Element<'_, Message> {
    // 文件状态显示
    let file_name = app
        .file_path
        .as_ref()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("未命名");
    let modified_indicator = if app.is_modified { " *" } else { "" };

    let file_status = text(format!("{file_name}{modified_indicator}")).size(13);

    // 工具栏
    let undo_button = if app.can_undo {
        button("↶ 撤销 (Ctrl+Z)")
            .on_press(Message::Undo)
            .padding(10)
    } else {
        button("↶ 撤销").padding(10)
    };

    let redo_button = if app.can_redo {
        button("↷ 重做 (Ctrl+Y)")
            .on_press(Message::Redo)
            .padding(10)
    } else {
        button("↷ 重做").padding(10)
    };

    let toolbar = row![
        button("📁 打开 (Ctrl+O)")
            .on_press(Message::FileOpen)
            .padding(10),
        button("💾 保存 (Ctrl+S)")
            .on_press(Message::FileSave)
            .padding(10),
        undo_button,
        redo_button,
        text(" "),
        file_status,
    ]
    .spacing(10)
    .align_y(alignment::Vertical::Center)
    .padding(5);

    // 编辑器 - 多行文本编辑器
    let editor = scrollable(
        container(
            text_editor(&app.editor_content)
                .on_action(Message::EditorAction)
                .padding(10),
        )
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::FillPortion(1))
    .height(Length::Fill);

    // 预览面板 - 显示 HTML 输出
    let preview_html = Renderer::render(app.buffer.content());
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

    // 状态栏
    let status_bar = text(&app.status_message).size(12);

    // 组装应用
    let app_layout = column![toolbar, content, status_bar]
        .spacing(10)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill);

    container(app_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// 生成动态窗口标题
fn window_title(app: &App) -> String {
    let modified = if app.is_modified { "* " } else { "" };
    match &app.file_path {
        Some(p) => format!(
            "{}{} — Lucid",
            modified,
            p.file_name().and_then(|n| n.to_str()).unwrap_or("文件")
        ),
        None => format!("{modified}未命名 — Lucid"),
    }
}

/// 应用启动函数 - 返回初始状态和可选的初始任务
fn boot() -> (App, Task<Message>) {
    (App::default(), Task::none())
}

/// 应用入口
pub fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title(window_title)
        .run()
}
