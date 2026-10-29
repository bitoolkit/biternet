use anyhow::{Context, Result};
use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, Stream, StreamExt};
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};

const MENU_SHOW_WINDOW: &str = "show_window";
const MENU_EXIT_APP: &str = "exit_app";
const TRAY_ICON: &[u8] = include_bytes!("../assets/icons/tray.png");

#[derive(Clone, Copy, Debug)]
pub enum Action {
    ShowWindow,
    Exit,
}

pub fn create() -> Result<TrayIcon> {
    let show = MenuItem::with_id(MENU_SHOW_WINDOW, "显示窗口", true, None);
    let exit = MenuItem::with_id(MENU_EXIT_APP, "退出程序", true, None);
    let separator = PredefinedMenuItem::separator();
    let menu = Menu::with_items(&[&show, &separator, &exit])?;
    let image = image::load_from_memory(TRAY_ICON).unwrap().into_rgba8();
    let icon = Icon::from_rgba(image.to_vec(), image.width(), image.height()).unwrap();

    TrayIconBuilder::new()
        .with_tooltip("BITERNET")
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .with_menu_on_left_click(!cfg!(target_os = "macos"))
        .build()
        .context("创建系统托盘失败")
}

pub fn events() -> impl Stream<Item = Action> {
    iced::stream::channel(16, async |mut output| {
        let (sender, mut receiver) = mpsc::unbounded();
        let menu_sender = sender.clone();
        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            let action = match event.id.as_ref() {
                MENU_SHOW_WINDOW => Action::ShowWindow,
                MENU_EXIT_APP => Action::Exit,
                _ => return,
            };

            let _ = menu_sender.unbounded_send(action);
        }));

        TrayIconEvent::set_event_handler(Some(move |event| {
            if activates_window(&event) {
                let _ = sender.unbounded_send(Action::ShowWindow);
            }
        }));

        while let Some(action) = receiver.next().await {
            if output.send(action).await.is_err() {
                break;
            }
        }
    })
}

const fn activates_window(event: &TrayIconEvent) -> bool {
    #[cfg(target_os = "windows")]
    {
        matches!(event, TrayIconEvent::DoubleClick { .. })
    }
    #[cfg(not(target_os = "windows"))]
    {
        matches!(
            event,
            TrayIconEvent::Click {
                button: tray_icon::MouseButton::Left,
                button_state: tray_icon::MouseButtonState::Up,
                ..
            }
        )
    }
}
