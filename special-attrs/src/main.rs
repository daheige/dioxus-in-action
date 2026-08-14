#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    println!("Hello, world!");
    // 在 main 函数中，通过 launch 函数运行整个程序，并传入根组件
    // 这里的主线程会被应用程序的事件循环所阻塞，直到触发整个程序的退出
    // dioxus::launch(Root); // 简单做法

    // 自定义配置
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new().with_window(
                dioxus::desktop::WindowBuilder::new()
                    .with_title("special-attrs")
                    .with_inner_size(dioxus::desktop::LogicalSize::new(640.0, 640.0)),
            ),
        )
        .launch(Root);
}

// 自定义post组件
// Props 组件条件渲染
#[component]
fn Post(title: String, content: String) -> Element {
    let css_style =
        r#".title { font-size: 18px; text-align: left;} .content{width:100%;text-align:left;}"#;
    rsx! {
        style { "{css_style}" }
        div {
            class: "card-content",
            hr {}
            h1 { class: "title", "文章：{title}" }
            p { class: "content", "内容：{content}" }
        }
    }
}

fn Root() -> Element {
    let blogs = vec![
        ("Hello World", "这是我的第一篇博客"),
        ("Dioxus", "hello,Dioxus"),
        ("rust", "rust demo"),
    ];

    let content = include_str!("../assets/app.html");
    // 引入本地css文件
    let app_css = include_str!("../assets/app.css");

    rsx! {
        style { "{app_css}" }
        div {
            class: "app",
            dangerous_inner_html: "{content}",
        }
        p {
            "开始遍历文章列表"
        }
        for (title, content) in blogs {
            div {
                class: "card",
                Post {
                    title: title.to_string(),
                    content: content.to_string()
                }
            }
        }
        input {
            oninput: move |event| {
                println!("新的内容被输入来:{:?}", event);
            }
        }
        button {
            onclick: move |event| {
                println!("按钮被点击了:{:?}", event);
            },
            "点击我"
        }
    }
}
