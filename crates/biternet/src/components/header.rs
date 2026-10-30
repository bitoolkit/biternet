use iced::widget::{button, container, mouse_area, row, space, text};
use iced::{Alignment, Border, Color, Element, Fill};

use crate::app::{App, Message, Overlay};
use crate::state::Session;
use crate::theme::{self, HEADING, MUTED, PRIMARY, SOFT_BLUE, SUCCESS};

pub fn view(app: &App) -> Element<'_, Message> {
    let (label, color, background) = match app.session {
        Session::Online(_) => ("已连接", SUCCESS, Color::from_rgb8(232, 245, 238)),
        Session::Initializing => ("检查中", PRIMARY, SOFT_BLUE),
        Session::LoggingIn { .. } => ("登录中", PRIMARY, SOFT_BLUE),
        Session::LoggingOut(_) => ("注销中", PRIMARY, SOFT_BLUE),
        Session::Offline => ("未连接", MUTED, Color::from_rgb8(238, 241, 247)),
    };

    let bar = row![
        text("校园网").size(18).font(HEADING),
        container(text(label).size(11).color(color))
            .padding([4, 8])
            .style(move |_| container::Style {
                background: Some(background.into()),
                border: Border {
                    radius: 20.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
        space::horizontal(),
        button(text("设置").size(12).height(Fill).center())
            .on_press_maybe(
                app.overlay
                    .is_none()
                    .then_some(Message::OverlayOpened(Some(Overlay::Settings))),
            )
            .style(theme::quiet_button)
            .height(28)
            .padding([0, 8]),
        button(text("×").size(18).height(Fill).width(Fill).center())
            .on_press_maybe(app.overlay.is_none().then_some(Message::CloseWindow))
            .style(theme::quiet_button)
            .height(28)
            .width(24)
            .padding(0),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .height(28);

    let mut area = mouse_area(bar);

    if let Some(drag) = app.drag() {
        area = area.on_press(drag);
    }

    area.into()
}
