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
        .with_title("hooks demo")
        .with_inner_size(LogicalSize::new(640, 640));
    dioxus_desktop::launch_cfg(app, config.with_window(window));
}

fn app(cx: Scope) -> Element {
    // rsx语法类似于jsx语法
    // RSX 会自动被转换为 HTML 结构，相对来说 RSX 更容易被表达和阅读

    // use_state 这一函数，它传递了 &cx 以及一个初始化函数,执行完毕后返回对应的T
    // use_state 是所有 Hook 函数中最基础的存在，也是最常用的
    let mut count = use_state(cx, || 0);
    let mut vote_count = use_state(cx, || 0);
    let mut count_a = use_state(cx, || 0);
    let mut count_b = use_state(cx, || 0);

    cx.render(rsx!(
        // 模拟加减操作
        h1 { "Counter_a: {count_a}" },
        button { onclick: move |_| count_a += 1, "a++" },
        button { onclick: move |_| count_a -= 1, "a--" },
        h1 { "Counter_b: {count_b}" },
        button { onclick: move |_| count_b += 1, "b++" },
        button { onclick: move |_| count_b -= 1, "b--" },
        p {
            "你一共点击了{count}次"
        },
        button {
            onclick: move |_|{
                count +=1;
            },
            "点击我"
        },
        p {
            "模拟点赞操作"
        },
        button {
            onclick: move |_|{
                vote_count +=1;
            },
            "up"
        },
        button {
            onclick: move |_|{
                vote_count -=1;
            },
            "down"
        },
        p { "vote_count: {vote_count}" },
    ))
}
