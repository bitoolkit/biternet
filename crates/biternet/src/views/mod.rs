mod login;
mod settings;
mod status;
mod update;

use iced::widget::{column, container, rule, space};
use iced::{Element, Fill, Padding};

use crate::app::{App, Message, Overlay};
use crate::components::{dialog, feedback, footer, header};
use crate::theme;

pub fn view(app: &App) -> Element<'_, Message> {
    let content = if app.session.online().is_some() {
        status::view(app)
    } else {
        login::view(app)
    };

    let notice = if app.overlay.is_none() {
        feedback::bar(app.notice.as_ref())
    } else {
        space().into()
    };

    let base = container(column![
        header::view(app),
        space().height(14),
        content,
        container(notice).height(28).center_y(28),
        rule::horizontal(1).style(theme::divider),
        footer::view(app),
    ])
    .padding(Padding {
        top: 14.0,
        bottom: 6.0,
        left: 16.0,
        right: 16.0,
    })
    .width(Fill)
    .height(Fill);

    let content = match app.overlay {
        Some(Overlay::Settings) => settings::view(app),
        Some(Overlay::Update) => update::view(
            app,
            app.update_info
                .as_ref()
                .expect("update dialog requires an available release"),
        ),
        Some(Overlay::Notice) => feedback::details(
            app,
            app.notice
                .as_ref()
                .expect("notice dialog requires a notice"),
        ),
        None => return base.into(),
    };

    dialog::view(base.into(), content)
}
