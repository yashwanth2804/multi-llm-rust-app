#![windows_subsystem = "windows"]
// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, env};

use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{Rect, WebViewBuilder};

#[derive(Clone, Copy)]
pub enum WebViewOrder {
    Gemini,
    Claude,
    ChatGpt,
}

impl WebViewOrder {
    pub fn get_url(&self) -> &str {
        match self {
            Self::Gemini => "https://gemini.google.com/",
            Self::Claude => "https://claude.ai",
            Self::ChatGpt => "https://chat.openai.com/",
        }
    }
}
fn main() -> wry::Result<()> {
    let mut web_views_map: HashMap<String, WebViewOrder> = HashMap::new();
    web_views_map.insert("Gi".to_string(), WebViewOrder::Gemini);
    web_views_map.insert("Cl".to_string(), WebViewOrder::Claude);
    web_views_map.insert("Ch".to_string(), WebViewOrder::ChatGpt);

    let mut web_views_order = vec![
        WebViewOrder::Gemini,
        WebViewOrder::ChatGpt,
        WebViewOrder::Claude,
    ];

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let orders: Vec<&str> = args[1].trim_start_matches("--").split(',').collect();
        web_views_order = orders
            .iter()
            .filter_map(|order| web_views_map.get(*order))
            .cloned()
            .collect::<Vec<_>>();
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1920, 1080))
        .build(&event_loop)
        .unwrap();
    let size = window.inner_size().to_logical::<u32>(window.scale_factor());
    let width = size.width;
    let height = size.height;

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let buffer_height = height - 80;
    let webview = WebViewBuilder::new_as_child(&window)
        .with_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(width / 3, buffer_height).into(),
        })
        .with_url(web_views_order[0].get_url())
        .build()?;
    let webview2 = WebViewBuilder::new_as_child(&window)
        .with_bounds(Rect {
            position: LogicalPosition::new(640 * 1, 0).into(),
            size: LogicalSize::new(width / 3, buffer_height).into(),
        })
        .with_url(web_views_order[1].get_url())
        .build()?;
    let webview3 = WebViewBuilder::new_as_child(&window)
        .with_bounds(Rect {
            position: LogicalPosition::new(640 * 2, 0).into(),
            size: LogicalSize::new(width / 3, buffer_height).into(),
        })
        .with_url(web_views_order[2].get_url())
        .build()?;

    // let _webview = builder
    //     .with_url("http://tauri.app")
    //     .with_drag_drop_handler(|e| {
    //         match e {
    //             wry::DragDropEvent::Enter { paths, position } => {
    //                 println!("DragEnter: {position:?} {paths:?} ")
    //             }
    //             wry::DragDropEvent::Over { position } => println!("DragOver: {position:?} "),
    //             wry::DragDropEvent::Drop { paths, position } => {
    //                 println!("DragDrop: {position:?} {paths:?} ")
    //             }
    //             wry::DragDropEvent::Leave => println!("DragLeave"),
    //             _ => {}
    //         }

    //         true
    //     })
    //     .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}
