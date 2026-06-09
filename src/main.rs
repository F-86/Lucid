// Lucid - GPUI 版本
// 基于 GPUI 0.2.2 构建的 Typora 风格 Markdown 编辑器

mod editor;
mod markdown;
mod ui;

use crate::editor::buffer::Buffer;
use crate::editor::history::History;
use crate::markdown::Renderer;
use crate::ui::{OpenFile, SaveFile, Undo, Redo};
use gpui::prelude::*;
use gpui::{
    Application, Context, div, FocusHandle, KeyDownEvent, KeyBinding,
    MouseButton, Render, rgb, ViewContext, Window, WindowOptions,
};
use std::path::PathBuf;

/// 应用主状态结构体
/// 替代原 iced 的 App struct，GPUI 风格：状态与渲染分离
pub struct LucidApp {
    // 编辑器核心模块（纯 Rust）
    buffer: Buffer,
    history: History,
    // UI 状态
    content: String,
    preview_html: String,
    file_path: Option<PathBuf>,
    is_modified: bool,
    status_message: String,
    can_undo: bool,
    can_redo: bool,
    // GPUI 特有：焦点管理（替代 iced 的 text_editor::Content）
    focus_handle: FocusHandle,
}

impl LucidApp {
    /// 初始化应用状态
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            buffer: Buffer::new(),
            history: History::new(),
            content: String::new(),
            preview_html: String::new(),
            file_path: None,
            is_modified: false,
            status_message: String::new(),
            can_undo: false,
            can_redo: false,
            focus_handle: cx.focus_handle(),
        }
    }

    /// 将内容同步到各个子模块，更新预览和历史
    fn sync_content(&mut self, cx: &mut Context<Self>) {
        let c = self.content.clone();
        self.buffer.set_content(&c);
        self.preview_html = Renderer::render(&c);
        self.is_modified = true;
        self.history.push(c);
        self.can_undo = self.history.can_undo();
        self.can_redo = self.history.can_redo();
        cx.notify();
    }

    /// 处理 OpenFile Action
    fn handle_open_file(&mut self, _: &OpenFile, _win: &mut Window, cx: &mut Context<Self>) {
        cx.spawn(|handle, mut cx| async move {
            let picked = rfd::AsyncFileDialog::new()
                .add_filter("Markdown", &["md", "txt"])
                .set_title("打开文件")
                .pick_file()
                .await;

            if let Some(h) = picked {
                let path = h.path().to_path_buf();
                match editor::file::read_file(&path).await {
                    Ok(content) => {
                        handle.update(&mut cx, |this, cx| {
                            this.buffer.set_content(&content);
                            this.preview_html = Renderer::render(&content);
                            this.content = content;
                            this.file_path = Some(path.clone());
                            this.is_modified = false;
                            this.history.clear();
                            this.can_undo = false;
                            this.can_redo = false;
                            let name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("未知文件")
                                .to_string();
                            this.status_message = format!("✓ 已打开：{name}");
                            cx.notify();
                        }).ok();
                    }
                    Err(e) => {
                        handle.update(&mut cx, |this, cx| {
                            this.status_message = format!("✗ 打开失败：{e}");
                            cx.notify();
                        }).ok();
                    }
                }
            }
        }).detach();
    }

    /// 处理 SaveFile Action
    fn handle_save_file(&mut self, _: &SaveFile, _win: &mut Window, cx: &mut Context<Self>) {
        let path = self.file_path.clone();
        let content = self.content.clone();
        cx.spawn(|handle, mut cx| async move {
            let save_path = match path {
                Some(p) => Some(p),
                None => rfd::AsyncFileDialog::new()
                    .add_filter("Markdown", &["md"])
                    .add_filter("文本文件", &["txt"])
                    .set_title("保存文件")
                    .save_file()
                    .await
                    .map(|h| h.path().to_path_buf()),
            };

            if let Some(p) = save_path {
                match editor::file::write_file(&p, &content).await {
                    Ok(()) => {
                        handle.update(&mut cx, |this, cx| {
                            this.file_path = Some(p);
                            this.is_modified = false;
                            this.status_message = "✓ 已保存".to_string();
                            cx.notify();
                        }).ok();
                    }
                    Err(e) => {
                        handle.update(&mut cx, |this, cx| {
                            this.status_message = format!("✗ 保存失败：{e}");
                            cx.notify();
                        }).ok();
                    }
                }
            }
        }).detach();
    }

    /// 处理 Undo Action
    fn handle_undo(&mut self, _: &Undo, _win: &mut Window, cx: &mut Context<Self>) {
        if let Some(prev) = self.history.undo() {
            self.buffer.set_content(&prev);
            self.preview_html = Renderer::render(&prev);
            self.content = prev;
            self.is_modified = true;
            self.can_undo = self.history.can_undo();
            self.can_redo = self.history.can_redo();
            self.status_message = "↶ 已撤销".to_string();
            cx.notify();
        } else {
            self.status_message = "⚠ 没有可撤销的操作".to_string();
            cx.notify();
        }
    }

    /// 处理 Redo Action
    fn handle_redo(&mut self, _: &Redo, _win: &mut Window, cx: &mut Context<Self>) {
        if let Some(next) = self.history.redo() {
            self.buffer.set_content(&next);
            self.preview_html = Renderer::render(&next);
            self.content = next;
            self.is_modified = true;
            self.can_undo = self.history.can_undo();
            self.can_redo = self.history.can_redo();
            self.status_message = "↷ 已重做".to_string();
            cx.notify();
        } else {
            self.status_message = "⚠ 没有可重做的操作".to_string();
            cx.notify();
        }
    }

    /// 渲染工具栏
    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_3()
            .p_3()
            .bg(rgb(0x282838))
            .border_b_1()
            .border_color(rgb(0x45475a))
            // 打开按钮
            .child(
                div()
                    .px_4().py_2()
                    .rounded("md")
                    .bg(rgb(0x4a9eff))
                    .cursor_pointer()
                    .on_click(cx.listener(|_, _evt, _win, _cx| {}))
                    .on_action(cx.listener(Self::handle_open_file))
                    .child("📁 打开 (⌘O)")
            )
            // 保存按钮
            .child(
                div()
                    .px_4().py_2()
                    .rounded("md")
                    .bg(rgb(0x4a9eff))
                    .cursor_pointer()
                    .on_action(cx.listener(Self::handle_save_file))
                    .child("💾 保存 (⌘S)")
            )
            // 撤销按钮
            .child({
                let bg = if self.can_undo { rgb(0x4a9eff) } else { rgb(0x45475a) };
                div()
                    .px_4().py_2()
                    .rounded("md")
                    .bg(bg)
                    .when(self.can_undo, |v| v.cursor_pointer())
                    .when(self.can_undo, |v| v.on_action(cx.listener(Self::handle_undo)))
                    .child("↶ 撤销 (⌘Z)")
            })
            // 重做按钮
            .child({
                let bg = if self.can_redo { rgb(0x4a9eff) } else { rgb(0x45475a) };
                div()
                    .px_4().py_2()
                    .rounded("md")
                    .bg(bg)
                    .when(self.can_redo, |v| v.cursor_pointer())
                    .when(self.can_redo, |v| v.on_action(cx.listener(Self::handle_redo)))
                    .child("↷ 重做 (⌘⇧Z)")
            })
    }

    /// 渲染编辑区（左栏）
    fn render_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .h_full()
            .bg(rgb(0x282838))
            .p_4()
            .track_focus(&self.focus_handle)
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _evt, win, _cx| {
                this.focus_handle.focus(win); // 鼠标点击获取焦点
            }))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _win, cx| {
                // Command 组合键由 Actions 系统处理，此处跳过
                if event.keystroke.modifiers.command {
                    return;
                }

                let key = &event.keystroke.key;
                match key.as_str() {
                    "backspace" => {
                        this.content.pop();
                        this.sync_content(cx);
                    }
                    "enter" => {
                        this.content.push('\n');
                        this.sync_content(cx);
                    }
                    _ if key.chars().count() == 1 => {
                        if let Some(c) = key.chars().next() {
                            this.content.push(c);
                            this.sync_content(cx);
                        }
                    }
                    _ => {}
                }
            }))
            .child(
                div()
                    .font_family("JetBrains Mono")
                    .text_sm()
                    .text_color(rgb(0xcdd6f4))
                    .child(self.content.clone())
            )
    }

    /// 渲染预览区（右栏）
    fn render_preview(&self) -> impl IntoElement {
        let display_text = if self.preview_html.is_empty() {
            "预览将在此显示".to_string()
        } else {
            self.preview_html.clone()
        };

        div()
            .flex_1()
            .h_full()
            .bg(rgb(0x1e1e2e))
            .p_4()
            .overflow_y_scroll()
            .child(
                div()
                    .font_family("JetBrains Mono")
                    .text_sm()
                    .text_color(rgb(0xcdd6f4))
                    .child(display_text)
            )
    }

    /// 渲染内容区（编辑器 + 预览）
    fn render_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .h_full()
            .flex()
            .gap_2()
            .p_2()
            .child(self.render_editor(cx))
            .child(self.render_preview())
    }

    /// 渲染状态栏
    fn render_status_bar(&self) -> impl IntoElement {
        div()
            .w_full()
            .px_4().py_2()
            .bg(rgb(0x282838))
            .border_t_1()
            .border_color(rgb(0x45475a))
            .text_xs()
            .text_color(rgb(0xb8bff0))
            .child(&self.status_message)
    }
}

/// 实现 Render trait（GPUI 的核心渲染接口，替代 iced 的 view fn）
impl Render for LucidApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("LucidApp")
            .track_focus(&self.focus_handle)
            // 注册 Action 处理器（替代 update() 中的 match 分支）
            .on_action(cx.listener(Self::handle_open_file))
            .on_action(cx.listener(Self::handle_save_file))
            .on_action(cx.listener(Self::handle_undo))
            .on_action(cx.listener(Self::handle_redo))
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x1e1e2e))
            .child(self.render_toolbar(cx))
            .child(self.render_content(cx))
            .child(self.render_status_bar())
    }
}

/// 应用入口（替代 `iced::application(...).run()`）
fn main() {
    Application::new().run(|cx: &mut gpui::App| {
        // 全局快捷键绑定（替代 subscription + event::listen()）
        cx.bind_keys([
            KeyBinding::new(&["cmd-o"], OpenFile, None),
            KeyBinding::new(&["cmd-s"], SaveFile, None),
            KeyBinding::new(&["cmd-z"], Undo, None),
            KeyBinding::new(&["cmd-shift-z"], Redo, None),
        ]);

        // 打开主窗口
        cx.open_window(
            WindowOptions::default(),
            |_window, cx| cx.new(|cx| LucidApp::new(cx)),
        ).unwrap();

        cx.activate(true);
    });
}
