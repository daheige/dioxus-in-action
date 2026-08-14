# dioxus in action

Dioxus 是一款用于构建跨平台用户界面的 Rust 框架。本实战案例基于 **Dioxus 0.7.10** 编写，包含多个独立的桌面端示例程序，帮助开发者快速掌握 Dioxus 0.7 的核心用法。

官方文档：https://dioxuslabs.com/learn/0.7/

---

## 目录

- [多平台支持](#多平台支持)
- [环境要求](#环境要求)
- [项目结构](#项目结构)
- [快速开始](#快速开始)
- [工作区成员](#工作区成员)
- [Dioxus 0.7 主要变化](#dioxus-07-主要变化)
- [示例代码](#示例代码)
- [官方资源](#官方资源)
- [学习资料](#学习资料)
- [Dioxus 功能点](#dioxus-功能点)
- [相关文档](#相关文档)

---

## 多平台支持

Dioxus 支持一套代码多端运行：

- 网页应用（使用 WASM 构建）
- 桌面应用（使用 Wry 构建）
- 移动应用（使用 Wry 构建）
- 终端应用（使用 Rink 构建）

> 本工作区所有示例默认均为桌面应用，运行时需要对应平台的 GUI 支持。

---

## 环境要求

- Rust 1.80+（建议使用最新稳定版）
- 桌面示例运行时需要对应平台的 GUI 支持
  - **Windows**：WebView2 运行时
  - **macOS**：WKWebView
  - **Linux**：WebKitGTK / gtk3

---

## 项目结构

```
dioxus-in-action/
├── Cargo.toml              # Workspace 根配置
├── readme.md               # 本文件（实战入门指南）
├── dioxus-0.7.md           # Dioxus 0.7.10 架构设计全面分析
├── dioxus-demo/            # 基础 RSX 语法与迭代渲染
├── window-ctrl/            # 窗口控制与事件处理
├── custom-html/            # 自定义 head 与样式
├── rsx-demo/               # RSX 属性、class、data 属性
├── page-design/            # 内嵌 style 与自定义 CSS
├── link-css/               # 引入本地/远程 CSS
├── page-render/            # 条件渲染与变量插值
├── props-render/           # Props 组件与条件渲染
├── iter-render/            # 列表渲染与 filter
├── special-attrs/          # dangerous_inner_html 与事件处理
└── hooks-demo/             # use_signal 状态管理
```

---

## 快速开始

```bash
# 克隆仓库
git clone https://github.com/daheige/dioxus-in-action.git
cd dioxus-in-action

# 编译整个工作区
cargo build --workspace

# 运行单个示例
cargo run -p dioxus-demo
cargo run -p hooks-demo
cargo run -p window-ctrl
```

### 运行指定示例

```bash
# 基础 RSX 与迭代渲染
cargo run -p dioxus-demo

# 窗口控制与事件
cargo run -p window-ctrl

# 状态管理
cargo run -p hooks-demo

# 列表渲染
cargo run -p iter-render

# 条件渲染
cargo run -p page-render

# Props 组件
cargo run -p props-render

# 自定义样式
cargo run -p page-design

# 引入 CSS
cargo run -p link-css

# 自定义 head
cargo run -p custom-html

# RSX 属性
cargo run -p rsx-demo

# 特殊属性与事件
cargo run -p special-attrs
```

---

## 工作区成员

| 示例 | 说明 |
|------|------|
| `dioxus-demo` | 基础 RSX 语法与迭代渲染 |
| `window-ctrl` | 窗口控制与事件处理 |
| `custom-html` | 自定义 head 与样式 |
| `rsx-demo` | RSX 属性、class、data 属性 |
| `page-design` | 内嵌 style 与自定义 CSS |
| `link-css` | 引入本地/远程 CSS |
| `page-render` | 条件渲染与变量插值 |
| `props-render` | Props 组件与条件渲染 |
| `iter-render` | 列表渲染与 filter |
| `special-attrs` | `dangerous_inner_html` 与事件处理 |
| `hooks-demo` | `use_signal` 状态管理 |

---

## Dioxus 0.7 主要变化

相较于 0.4/0.5，0.7 有较大的 API 调整：

- 组件函数不再接收 `cx: Scope` 参数
- `cx.render(rsx!(...))` 简化为 `rsx!(...)`
- `use_state(cx, || ...)` 改为 `use_signal(|| ...)`
- 桌面应用启动改为 `LaunchBuilder::desktop().with_cfg(...).launch(App)`
- `Cargo.toml` 需要启用 `features = ["desktop"]`
- 新增 `Stores` API 用于嵌套结构响应式状态
- 全栈能力整合：`#[server]`、`#[route]`、SSR、Hydration

---

## 示例代码

```rust
#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new().with_window(
                dioxus::desktop::WindowBuilder::new()
                    .with_title("dioxus demo")
                    .with_inner_size(dioxus::desktop::LogicalSize::new(640.0, 640.0)),
            ),
        )
        .launch(App);
}

fn App() -> Element {
    rsx! {
        div { "Hello, world!" }
        p { "hello, dioxus" }
    }
}
```

上面的代码会渲染为：

```html
<div>Hello, world!</div>
<p>hello, dioxus</p>
```

---

## 官方资源

- GitHub：https://github.com/dioxuslabs/dioxus
- 中文 README：https://github.com/DioxusLabs/dioxus/blob/master/notes/README/ZH_CN.md
- 0.7 文档：https://dioxuslabs.com/learn/0.7/
- API 文档：https://docs.rs/dioxus/0.7.10/dioxus/
- 官方示例：https://github.com/DioxusLabs/dioxus/tree/v0.6/examples

---

## 学习资料

### HTML5 基础
- https://www.runoob.com/html/html5-intro.html
- https://zhuanlan.zhihu.com/p/459873347

### CSS 基础
- https://www.runoob.com/css/css-tutorial.html
- https://www.cainiaojc.com/css3/css3-tutorial.html

### JSX 语法基础
- JSX 是一种 JavaScript 的语法扩展（eXtension），也在很多地方称之为 JavaScript XML；
- 它用于描述 UI 界面，并且完全可以和 JavaScript 融合在一起使用；
- 它不同于 Vue 中的模块语法，不需要专门学习模块语法中的一些指令（比如 v-for、v-if、v-else、v-bind）。

参考：https://juejin.cn/post/6996214286292877326

---

## Dioxus 功能点

- 参照 React 设计，使得相关开发人员过渡简单。
- 强大状态管理系统以及易用的 Hooks 设计。
- 桌面应用原生支持，提供部分常用 API 可调用。
- 简洁的 RSX 界面声明格式，比 HTML 更加易读。
- 全栈能力：类型安全的 Server Functions、SSR、Hydration。
- 细粒度响应式：Signals 自动订阅，精准更新。
- 热重载 + Subsecond 热补丁，缩短开发反馈循环。

---

## 相关文档

- [Dioxus 0.7.10 架构设计全面分析](./dioxus-0.7.md)：Virtual DOM、Signals、RSX、多平台渲染器、全栈架构、Blitz、Subsecond 热补丁等深度解析。
