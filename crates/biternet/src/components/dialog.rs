use iced::widget::{button, container, mouse_area, opaque, row, space, stack, text};
use iced::{Alignment, Color, Element, Fill};

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

pub fn header<'a>(title: &'a str, feedback: Option<Element<'a, Message>>) -> Element<'a, Message> {
    let mut header = row![text(title).size(18).font(HEADING).width(Fill)]
        .spacing(8)
        .align_y(Alignment::Center);

    if let Some(feedback) = feedback {
        header = header.push(feedback);
    }

    header
        .push(
            button(text("×").size(20).height(Fill).width(Fill).center())
                .on_press(Message::OverlayOpened(None))
                .style(theme::quiet_button)
                .width(28)
                .height(28)
                .padding(0),
        )
        .into()
}
