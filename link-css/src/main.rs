#![allow(non_snake_case)]

use dioxus::prelude::*;

// 用于自定义窗口
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

fn main() {
    println!("Hello, world!");
    // 在 main 函数中，通过 lanuch 函数运行整个程序，并传入根组件app
    // 这里的主线程会被应用程序的事件循环所阻塞，直到触发整个程序的退出
    // dioxus_desktop::launch(app); // 简单做法

    // 自定义配置
    let config = Config::new();
    // 创建窗口并设置窗口标题和窗口大小
    let window = WindowBuilder::new()
        .with_title("rsx demo")
        .with_inner_size(LogicalSize::new(640, 640));
    dioxus_desktop::launch_cfg(app, config.with_window(window));
}

fn app(cx: Scope) -> Element {
    // rsx语法类似于jsx语法
    // RSX 会自动被转换为 HTML 结构，相对来说 RSX 更容易被表达和阅读

    // 引入外部css样式
    let css_url = "https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css";
    cx.render(rsx!(
        // 引入外部css样式
        link {
            rel:"stylesheet", // 样式表
            href:"{css_url}"
        },
        // 引入本地css文件
        style {
            include_str!("../assets/app.css")
        },

        div{
            "class":"app",
            "本地样式css，hello"
        }
        hr{},
        div {
            // 使用外部样式
            "class": "container",
            div {
                "class": "card",
                div {
                    class: "card-content",
                    "应用外部css样式，Card Content"
                }
            }
        },
        p {
            // 行内样式定义
            "style":"text-align:center;",
            "hello"
        }
    ))
}
