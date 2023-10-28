#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    println!("Hello, world!");
    // 在 main 函数中，通过 lanuch 函数运行整个程序，并传入根组件app
    // 这里的主线程会被应用程序的事件循环所阻塞，直到触发整个程序的退出
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    // rsx语法类似于jsx语法
    // RSX 会自动被转换为 HTML 结构，相对来说 RSX 更容易被表达和阅读
    cx.render(rsx!(
        div {
            "style": "text-align:center;",
            "Hello, world!"
        },
        p{
            "hello,dioxus"
        },
        ol{
            li {
                "fist item"
            },
            li {
                "second item"
            },
            li {
                "third item"
            }
        },
        p{
            "这是一个段落文本"
        },
        // 遍历
        p{
            "开始遍历"
        },
        div{
            (0..3).map(|i|rsx!{
                div{
                    "current i = {i}"
                }
            })
        }
    ))
}
