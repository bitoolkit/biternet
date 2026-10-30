use iced::widget::{button, container, mouse_area, opaque, row, space, stack, text};
use iced::{Alignment, Color, Element, Fill, Padding};

use crate::app::Message;
use crate::theme::{self, HEADING};

pub fn view<'a>(base: Element<'a, Message>, content: Element<'a, Message>) -> Element<'a, Message> {
    let backdrop =
        mouse_area(
            container(space())
                .width(Fill)
                .height(Fill)
                .style(|_| container::Style {
                    background: Some(Color::from_rgba8(35, 49, 73, 0.16).into()),
                    ..Default::default()
                }),
        )
        .on_press(Message::OverlayOpened(None));

    let panel = container(content)
        .padding(20)
        .width(Fill)
        .style(theme::dialog_panel);

    stack![
        base,
        opaque(backdrop),
        container(opaque(panel))
            .padding(16)
            .center_x(Fill)
            .center_y(Fill),
    ]
    .into()
}

pub fn header<'a>(
    title: &'a str,
    feedback: Option<Element<'a, Message>>,
    drag: Option<Message>,
) -> Element<'a, Message> {
    // Only the title strip starts a drag: wrapping the close button would let
    // its release-on-press trigger alongside the window drag.
    let mut strip = row![text(title).size(18).font(HEADING)]
        .padding(Padding {
            right: 8.0,
            ..Padding::ZERO
        })
        .width(Fill)
        .align_y(Alignment::Center);

    if let Some(feedback) = feedback {
        strip = strip.push(feedback);
    }

    let mut area = mouse_area(strip);

    if let Some(drag) = drag {
        area = area.on_press(drag);
    }

    row![
        area,
        button(text("×").size(20).height(Fill).width(Fill).center())
            .on_press(Message::OverlayOpened(None))
            .style(theme::quiet_button)
            .width(28)
            .height(28)
            .padding(0),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}
