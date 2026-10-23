use iced::widget::{button, column, container, row, space, text};
use iced::{Alignment, Border, Element, Fill};

use crate::app::{App, Message};
use crate::state::Session;
use crate::theme::{self, HEADING, MUTED, PRIMARY, SOFT_BLUE};

pub fn view(app: &App) -> Element<'_, Message> {
    let info = app
        .session
        .online()
        .expect("status view requires an online session");
    let identity = column![
        text(&info.username)
            .size(16)
            .font(HEADING)
            .width(Fill)
            .wrapping(text::Wrapping::WordOrGlyph),
        row![
            text("IP").size(11).color(MUTED),
            text(&info.ip)
                .size(12)
                .color(MUTED)
                .width(Fill)
                .wrapping(text::Wrapping::WordOrGlyph),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    ]
    .spacing(3);
    let usage = container(
        column![
            row![
                text("已用流量").size(11).color(MUTED),
                space::horizontal(),
                text(&info.used_flux).size(20).font(HEADING).color(PRIMARY),
            ]
            .spacing(8)
            .width(Fill)
            .align_y(Alignment::Center),
            row![
                text("余额（元）").size(11).color(MUTED),
                space::horizontal(),
                text(&info.balance).size(14).font(HEADING),
            ]
            .spacing(8)
            .width(Fill)
            .align_y(Alignment::Center),
        ]
        .spacing(8),
    )
    .padding([12, 14])
    .width(Fill)
    .style(|_| container::Style {
        background: Some(SOFT_BLUE.into()),
        border: Border {
            radius: 12.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let duration = row![
        text("已用时长").size(12).color(MUTED),
        text(&info.used_duration)
            .size(12)
            .width(Fill)
            .align_x(iced::alignment::Horizontal::Right),
    ]
    .spacing(8)
    .align_y(Alignment::Center);
    let label = if matches!(app.session, Session::LoggingOut(_)) {
        "正在注销…"
    } else {
        "注销登录"
    };

    column![
        identity,
        space().height(12),
        usage,
        space().height(12),
        duration,
        space::vertical(),
        button(text(label).height(Fill).width(Fill).center())
            .on_press_maybe(app.can_logout().then_some(Message::Logout))
            .style(theme::logout_button)
            .width(Fill)
            .height(34)
            .padding(0),
    ]
    .height(Fill)
    .into()
}
