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
        .with_title("special-attrs")
        .with_inner_size(LogicalSize::new(640, 640));
    dioxus_desktop::launch_cfg(root, config.with_window(window));
}

// 自定义post组件
// Props 组件条件渲染
#[derive(Props, PartialEq)]
struct PostProps {
    title: String,
    content: String,
}

fn Post(cx: Scope<PostProps>) -> Element {
    let css_style =
        r#".title { font-size: 18px; text-align: left;} .content{width:100%;text-align:left;}"#;
    cx.render(rsx!(
        style {
            "{css_style}"
        },
        div {
            class: "card-content",
            hr {},
            h1 {"class":"title","文章：{cx.props.title}" },
            p {"class":"content","内容：{cx.props.content}" }
        }
    ))
}

fn root(cx: Scope) -> Element {
    let blogs = vec![
        ("Hello World", "这是我的第一篇博客"),
        ("Dioxus", "hello,Dioxus"),
        ("rust", "rust demo"),
    ];
    let ele = blogs.iter().map(|(title, content)| {
        rsx! {
            div{
                "class": "card",
                Post {
                    title:title.to_string(),
                    content: content.to_string()
                }
            }
        }
    });

    let content = include_str!("../assets/app.html");
    cx.render(rsx! {
         // 引入本地css文件
        style {
            include_str!("../assets/app.css")
        },
        div {
            "class":"app",
            dangerous_inner_html: "{content}",
        },
        p {
            "开始遍历文章列表"
        },
        ele,
        input {
            oninput:|event| {
                println!("新的内容被输入来:{:?}",event);
            }
        },
        button {
            onclick:|event|{
                println!("按钮被点击了:{:?}",event);
            },
            "点击我"
        }
    })
}
