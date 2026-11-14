# Snippy Rust 复刻计划

目标：clean-room 复刻 SnipLite 的截图、标注、贴图、OCR、二维码与翻译能力；单一 Rust 代码库支持 Windows、macOS、Linux。

## 技术路线

- UI：eframe/egui，避免分别维护 WinForms、AppKit、GTK。
- 截图：screenshots；平台差异封装在 capture 模块。
- 图像：image；标注使用可回放命令列表。
- 设置：directories + serde_json。
- OCR、QR、翻译：可替换后端，失败时可解释降级。

## 里程碑

1. M0：主屏截图、预览、PNG 保存、配置路径、最小测试。
2. M1：多屏选区、DPI、Esc 取消、Enter 确认、剪贴板、快捷键。
3. M2：矩形、椭圆、箭头、画笔、荧光笔、文字、马赛克、橡皮擦、撤销重做。
4. M3：托盘、置顶贴图、移动缩放、透明度、隐藏恢复。
5. M4：QR/条码、离线 OCR、本地模型与平台引擎回退。
6. M5：MyMemory/Bing/Google/Azure 翻译、设置页、更新、签名发布包。

## 边界

- 源项目为 MIT；本仓库从零实现，不复制原实现。
- 不默认上传截图；翻译仅在用户点击后发送识别文字。
- OCR 模型单独评估许可证、体积、离线更新策略。

## 当前状态

M0 骨架已提交：窗口、主屏截图、预览、PNG 保存、设置目录读取。当前开发机没有 Rust toolchain；安装 stable 后运行 cargo test 与 cargo run。
