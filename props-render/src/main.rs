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

// Props 组件条件渲染
#[component]
fn App(account_state: bool) -> Element {
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
    let page = match account_state {
        true => rsx!(
            div {
                // 作用于整个div内容居中
                style: "text-align:left;",
                p {
                    style: "margin-bottom:1px;",
                    "your desc:"
                }
                ul {
                    li { "name:{name}" }
                    li { "age:{age}" }
                    li { "status:{adult}" }
                }
                div {
                    "数组:{v:?}"
                }
            }
            div {
                // 通过布尔值来渲染元素
                {show_title.then(|| rsx!(
                    p {
                        "this title: rsx"
                    }
                ))}
            }
        ),
        false => rsx!(
            div {
                // 作用于整个div内容居中
                style: "text-align:left;",
                p {
                    style: "margin-bottom:1px;",
                    "no desc"
                }
            }
        ),
    };

    rsx! { {page} }
}

fn Root() -> Element {
    // 渲染app组件
    rsx! { App { account_state: true } }
}
