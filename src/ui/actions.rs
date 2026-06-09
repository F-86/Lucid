// UI Actions - 替代 iced Message enum
// 使用 GPUI 的 actions! 宏定义应用级别的 Actions
use gpui::actions;

// 声明应用级别的 Actions
// 命名空间 "lucid" 避免与其他模块冲突
actions!(lucid, [OpenFile, SaveFile, Undo, Redo]);
