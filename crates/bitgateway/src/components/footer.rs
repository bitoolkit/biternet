use iced::widget::{button, row, space, text};
use iced::{Alignment, Element, Fill};

use crate::app::{App, Message, Overlay};
use crate::theme::{self, MUTED};

pub fn view(app: &App) -> Element<'_, Message> {
    let enabled = app.overlay.is_none();
    let mut line = row![
        button(
            text(concat!("BITGATEWAY v", env!("CARGO_PKG_VERSION")))
                .size(11)
                .height(Fill)
                .center(),
        )
        .on_press_maybe(enabled.then_some(Message::OpenRepository))
        .style(|_, _| button::Style {
            text_color: MUTED,
            ..Default::default()
        })
        .height(24)
        .padding(0),
        space::horizontal(),
    ]
    .height(24)
    .align_y(Alignment::Center);

    if app.update_info.is_some() {
        line = line.push(
            button(text("有更新 ↗").size(11).height(Fill).center())
                .on_press_maybe(enabled.then_some(Message::OverlayOpened(Some(Overlay::Update))))
                .style(theme::update_button)
                .height(24)
                .padding([0, 8]),
        );
    }

    line.into()
}
