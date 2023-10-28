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

// Props 组件条件渲染
#[derive(Props, PartialEq)]
struct AppProps {
    account_state: bool, // 是否在线
}

// 对应的 AppProps 的App组件
fn App(cx: Scope<AppProps>) -> Element {
    // rsx语法类似于jsx语法
    // RSX 会自动被转换为 HTML 结构，相对来说 RSX 更容易被表达和阅读

    // 动态变量渲染
    let name = "daheige";

    // 你能渲染的类型有很多，只要它实现了 Display trait 特征
    let age = 33;
    let adult = true;

    // Debug特征
    let v = vec![1, 2, 3, 4, 5];

    let show_title = true;

    // 根据props组件属性值来渲染不同的页面
    let page = match cx.props.account_state {
        true => rsx!(
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
            },
            div{
                // 通过布尔值来渲染元素
                show_title.then(|| cx.render(rsx!(
                    p {
                        "this title: rsx"
                    }
                )))
            }
        ),
        false => rsx!(
            div {
                // 作用于整个div内容居中
                "style":"text-align:left;",
                p {
                    "style":"margin-bottom:1px;",
                    "no desc"
                }
            }
        ),
    };

    cx.render(rsx!(page))
}

fn root(cx: Scope) -> Element {
    // 渲染app组件
    cx.render(rsx!(App {
        account_state: true,
    }))
}
