#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    println!("Hello, world!");
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    // rsx语法类似于jsx语法
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
