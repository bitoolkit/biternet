use iced::widget::{button, column, row, space, text, toggler};
use iced::{Alignment, Element, Fill, Theme};

use crate::app::{App, Message, Overlay};
use crate::components::dialog;
use crate::theme::{self, MUTED, TEXT};

pub fn view(app: &App) -> Element<'_, Message> {
    let saved: Element<'_, Message> = match &app.notice {
        Some(notice) if notice.detail.is_some() => {
            button(text("保存失败").size(11).height(Fill).center())
                .on_press(Message::OverlayOpened(Some(Overlay::Notice)))
                .style(|theme: &Theme, _| button::Style {
                    text_color: theme.palette().danger,
                    ..Default::default()
                })
                .height(28)
                .padding(0)
                .into()
        }
        Some(_) => text("已保存").size(11).style(text::success).into(),
        None => space().into(),
    };

    column![
        dialog::header("设置", Some(saved), app.drag()),
        space().height(16),
        row![
            text("开机启动").size(13).width(Fill),
            toggler(app.settings.auto_start)
                .size(20)
                .style(theme::toggle)
                .on_toggle(Message::AutoStartChanged),
        ]
        .align_y(Alignment::Center),
        space().height(16),
        row![
            column![
                text("启动时隐藏窗口")
                    .size(13)
                    .color(if app.settings.auto_start { TEXT } else { MUTED }),
                text("仅在托盘显示").size(11).color(MUTED),
            ]
            .spacing(2)
            .width(Fill),
            toggler(app.settings.silent_start)
                .size(20)
                .style(theme::toggle)
                .on_toggle_maybe(
                    app.settings
                        .auto_start
                        .then_some(Message::SilentStartChanged)
                ),
        ]
        .align_y(Alignment::Start),
    ]
    .into()
}
