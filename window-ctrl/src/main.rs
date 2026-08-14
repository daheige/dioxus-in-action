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
                    .with_title("dioxus demo")
                    .with_inner_size(dioxus::desktop::LogicalSize::new(640.0, 640.0)),
            ),
        )
        .launch(App);
}

fn App() -> Element {
    // rsx语法类似于jsx语法
    // RSX 会自动被转换为 HTML 结构，相对来说 RSX 更容易被表达和阅读
    // 获取 window
    let win = dioxus::desktop::window();
    // win.set_decorations(false); // 设置窗口无边框

    rsx! {
        div {
            a {
                class: "close",
                onmousedown: move |e| { e.stop_propagation(); },
                onclick: move |_| { win.close() },
                "点击关闭窗口"
            }
        }
        div {
            style: "text-align:center;",
            "Hello, world!",
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
            "开始遍历"
        }
        div {
            for i in 0..3 {
                div { "current i = {i}" }
            }
        }
    }
}
