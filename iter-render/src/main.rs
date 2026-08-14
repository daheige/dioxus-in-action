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
                    .with_title("rsx demo")
                    .with_inner_size(dioxus::desktop::LogicalSize::new(640.0, 640.0)),
            ),
        )
        .launch(Root);
}

// PostProps自定义组件
// Props 组件条件渲染
#[component]
fn Post(title: String, content: String) -> Element {
    let css_style =
        r#".title { font-size: 18px; text-align: center;} .content{width:100%;text-align:left;}"#;
    rsx! {
        style { "{css_style}" }
        div {
            class: "card-content",
            h1 { class: "title", "{title}" }
            p { class: "content", "{content}" }
        }
    }
}

fn Root() -> Element {
    let blogs: Vec<(&'static str, &'static str)> = vec![
        ("Hello World", "这是我的第一篇博客"),
        ("Dioxus", "hello,Dioxus"),
        ("rust", "rust demo"),
    ];

    let names = vec!["jim", "bob", "jane", "doe", "jake"];

    rsx! {
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css"
        }
        div {
            class: "container",
            for (title, content) in blogs {
                div {
                    class: "card",
                    Post {
                        title: title.to_string(),
                        content: content.to_string()
                    }
                }
            }
            p {
                "filter for names"
            }
            ul {
                for name in names.into_iter().filter(|name| name.starts_with("j")) {
                    li { "data-key": "{name}", "current name:{name}" }
                }
            }
        }
    }
}
