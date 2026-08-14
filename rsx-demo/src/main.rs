#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    println!("Hello, world!");
    // 在 main 函数中，通过 launch 函数运行整个程序，并传入根组件 App
    // 这里的主线程会被应用程序的事件循环所阻塞，直到触发整个程序的退出
    // dioxus::launch(App); // 简单做法

    // 自定义配置
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new().with_window(
                dioxus::desktop::WindowBuilder::new()
                    .with_title("rsx demo")
                    .with_inner_size(dioxus::desktop::LogicalSize::new(640.0, 640.0)),
            ),
        )
        .launch(App);
}

fn App() -> Element {
    // rsx语法类似于jsx语法
    // RSX 会自动被转换为 HTML 结构，相对来说 RSX 更容易被表达和阅读
    rsx! {
        div {
            // 作用于整个div内容居中
            style: "text-align:center;",
            h1 { "hello dioxus" }
            a {
                href: "https://www.dioxus.cn/",
                "dioxus中文网"
            }
            // 转换为 <p class="content">hello,world</p>
            p {
                class: "content",
                "hello,world"
            }
        }
        // 自定义data属性 转换为<div data-count="10">hello,world</div>
        div {
            "data-count": "10",
            "hello,world"
        }
    }
}
