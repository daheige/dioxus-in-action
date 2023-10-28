# dioxus in action
Dioxus 是一款用于构建跨平台用户界面的框架。

# 多平台支持
- 网页应用（使用 WASM 构建）
- 桌面应用（使用 Wry 构建）
- 移动应用（使用 Wry 构建）
- 终端应用（使用 Rink 构建）

# dioxus官网
- https://github.com/dioxuslabs/dioxus
- https://github.com/DioxusLabs/dioxus/blob/master/notes/README/ZH_CN.md

# 使用手册
- https://dioxuslabs.com/learn/0.4/
- https://www.dioxus.cn/docs/quick-start/rsx
- https://www.dioxus.cn/docs/intro

# dioxus功能点
- 参照 React 设计，使得相关开发人员过渡简单。
- 强大状态管理系统以及易用的 Hooks 设计。
- 桌面应用原生支持，提供部分常用 API 可调用。
- 简洁的 RSX 界面声明格式，比 HTML 更加易读。

# dioxus rsx语法转换为html规则
```rust
// 通过rsx!宏的方式编写rsx语法
fn app(cx: Scope) -> Element {
    cx.render(rsx! ( 
        div { "Hello, world!" }
    ))    
}
```
上面的代码会转换为 HTML 中的元素：
```html
<div>Hello, world!</div>
```

# jsx语法基础
- jsx是一种JavaScript的语法扩展（eXtension），也在很多地方称之为JavaScript XML，因为看起来就是一段XML语法；
- 它用于描述我们的UI界面，并且其完成可以和JavaScript融合在一起使用；
- 它不同于Vue中的模块语法，你不需要专门学习模块语法中的一些指令（比如v-for、v-if、v-else、v-bind）；

参考：https://juejin.cn/post/6996214286292877326
