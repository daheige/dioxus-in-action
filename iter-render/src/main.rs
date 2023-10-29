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
    dioxus_desktop::launch_cfg(root, config.with_window(window));
}

// PostProps自定义组件
// Props 组件条件渲染
// 通过Props属性标记
#[derive(Props, PartialEq)]
struct PostProps {
    title: String,
    content: String,
}

fn Post(cx: Scope<PostProps>) -> Element {
    let css_style =
        r#".title { font-size: 18px; text-align: center;} .content{width:100%;text-align:left;}"#;
    cx.render(rsx!(
        style {
            "{css_style}"
        },
        div {
            class: "card-content",
            h1 {"class":"title","{cx.props.title}" },
            p {"class":"content","{cx.props.content}" }
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

    let names = vec!["jim", "bob", "jane", "doe", "jake"];

    // 采用filter进行过滤，然后采用map进行逐个遍历
    // 对于li每个项，采用来自定义属性data-key来区分
    let name_ele = names
        .iter()
        .filter(|name| {
            name.starts_with("j") // 过滤操作
        })
        .map(|name| rsx!(li{"data-key":"{name}","current name:{name}"}));

    cx.render(rsx! {
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css"
        },
        div {
            "class": "container",
            ele,
            p {
                "filter for names"
            },
            ul {
                name_ele
            }
        }
    })
}
