**🌱Seedling**
一件小事是一粒种子，勾选=浇水，长期看长成习惯森林。

该项目的主要目的，是直接统计md文档中具有的待办事项，可以作为平时日常文档管理的无侵入格式化。

一般会统计exe的同文件夹下的所有`md文档`中是否含有任务清单，有则会在首行和尾行添加类似：



> [███░░░░░░░] ⚡ 今日进度 1/3 | ⏳ 未完成 <u>2</u> | ✅ 已完成 1 · _By Seedling_ 🌱
>
> - [x] 架构设计讨论
> - [ ] Coding 需求
> - [ ] 打包测试CI/CD
>
> [███░░░░░░░] ⚡ 今日进度 1/3 | ⏳ 未完成 <u>2</u> | ✅ 已完成 1 · _By Seedling_ 🌱



**seedling-md**

- 扫描自身所在目录的 `*.md` 文件，若存在任务清单（`- [ ]`/`- [x]` 或 `* [ ]`/`* [x]`），在文档首行插入/更新统计信息。
- 现已同时在文档尾行插入/更新同款统计行，首尾对齐强化可视性。
- 统计行包含进度条、未完成与已完成计数，整体加粗；未完成的数字使用下划线强调；品牌为斜体加粗：`_By Seedling_ 🌱`。
- 保留原文件换行风格（LF/CRLF）与 BOM；无任务时不插入、不修改。

**打包命令**
- 进入项目目录：`cd seedling-md`
- 构建发布版：`cargo clean; cargo build --release`
- 可执行文件位置：`seedling-md/target/release/seedling-md.exe`

**使用**

- 目前只有windows版本，若需要可以自行手动进行编译打包。
- 将 `seedling-md.exe` 放入需要处理的 Markdown 目录后运行，即可批量更新首尾统计。

**设置 EXE 图标（Windows）**
- Windows 可执行文件图标需使用 `.ico` 格式，不能直接使用 `.svg`。
- 项目已加入图标嵌入流程（`build.rs` + `winres`）；当存在 `img/seedling.ico` 时，打包将自动嵌入图标。
- 将 `img/seedling.svg` 转换为 `img/seedling.ico` 的简便方法：
  - 使用 Inkscape 或任意在线转换工具，导出包含多尺寸的 ICO（建议 16/32/48/64/128/256）。
  - 或用 ImageMagick：`magick seedling.svg -define icon:auto-resize=64,128,256 seedling.ico`
  - 完成后放置到 `img/seedling.ico`，重新执行 `cargo build --release`。

![Logo](.\seedling-md\assets\seedling.png)