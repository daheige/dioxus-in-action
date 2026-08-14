#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    println!("Hello, world!");
    // 在 main 函数中，通过 launch 函数运行整个程序，并传入根组件 App
    // 这里的主线程会被应用程序的事件循环所阻塞，直到触发整个程序的退出
    // dioxus::launch(App); // 简单做法

    // 自定义配置header头，并设置head样式style
    let config = dioxus::desktop::Config::new().with_custom_head(
        r#"
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <style>body { background-color: green; }</style>
        "#
        .into(),
    );

    // 创建窗口并设置窗口标题和窗口大小
    let window = dioxus::desktop::WindowBuilder::new()
        .with_title("dioxus demo")
        .with_inner_size(dioxus::desktop::LogicalSize::new(640.0, 640.0));

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config.with_window(window))
        .launch(App);
}

fn App() -> Element {
    // rsx语法类似于jsx语法
    // RSX 会自动被转换为 HTML 结构，相对来说 RSX 更容易被表达和阅读
    rsx! {
        div {
            style: "text-align:center;",
            "Hello, world!"
        }
        p {
            "hello,dioxus"
        }
        ol {
            li { "fist item" }
            li { "second item" }
            li { "third item" }
        }
        p {
            "这是一个段落文本"
        }
        // 遍历
        p {
            "开始遍历0-4"
        }
        div {
            for i in 0..5 {
                div { "current i = {i}" }
            }
        }
    }
}
