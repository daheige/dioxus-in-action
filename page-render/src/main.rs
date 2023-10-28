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

struct AppProps {
    account_state: bool, // 是否在线
}

fn app(cx: Scope) -> Element {
    // rsx语法类似于jsx语法
    // RSX 会自动被转换为 HTML 结构，相对来说 RSX 更容易被表达和阅读

    // 动态变量渲染
    let name = "daheige";

    // 你能渲染的类型有很多，只要它实现了 Display trait 特征
    let age = 33;
    let adult = true;

    // Debug特征
    let v = vec![1, 2, 3, 4, 5];

    let app_pops = AppProps {
        account_state: true,
    };
    if app_pops.account_state {
        cx.render(rsx!(
            div {
                // 作用于整个div内容居中
                "style":"text-align:left;",
                p {
                    "style":"margin-bottom:1px;",
                    "your desc:"
                },
                ul {
                    li {
                        "name:{name}"
                    },
                    li {
                        "age:{age}"
                    },
                    li {
                        "status:{adult}"
                    }
                },
                div {
                    "数组:{v:?}"
                }
            }
        ))
    } else {
        cx.render(rsx!(
            div {
                // 作用于整个div内容居中
                "style":"text-align:left;",
                p {
                    "style":"margin-bottom:1px;",
                    "no desc"
                }
            }
        ))
    }
}
