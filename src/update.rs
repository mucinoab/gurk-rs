use crate::app::{App, Event};
use crate::environment::{Environment, SignalManager, TerminalKind};
use crate::storage::Storage;
use crate::ui;

use crossterm::event::{KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use log::error;
use tui::backend::Backend;

/// Main event loop handler.
///
/// Called on each incoming `event`.
///
/// Returns either an `App` or `None` in case the application should quit.
pub async fn update<SM, S, T, B>(
    mut app: App,
    event: Event,
    env: &mut Environment<SM, S, T, B>,
) -> anyhow::Result<Option<App>>
where
    SM: SignalManager,
    S: Storage,
    T: TerminalKind<B>,
    B: Backend,
{
    match event {
        Event::Click(event) => match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let col = event.column;
                let row = event.row;

                let size = env.terminal.size();

                if let Some(channel_idx) = ui::coords_within_channels_view(size, col, row)
                    .map(|(_, row)| row as usize)
                    .filter(|&idx| idx < app.data.channels.items.len())
                {
                    app.data.channels.state.select(Some(channel_idx as usize));
                    if app.reset_unread_messages() {
                        app.save().unwrap();
                    }
                }
            }
            MouseEventKind::ScrollUp => {
                if event.column < env.terminal.size().width / ui::CHANNEL_VIEW_RATIO as u16 {
                    app.select_previous_channel()
                } else {
                    app.on_pgup()
                }
            }
            MouseEventKind::ScrollDown => {
                if event.column < env.terminal.size().width / ui::CHANNEL_VIEW_RATIO as u16 {
                    app.select_next_channel()
                } else {
                    app.on_pgdn()
                }
            }
            _ => {}
        },
        Event::Input(event) => match event.code {
            KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                return Ok(None);
            }
            KeyCode::Left => {
                if event
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
                {
                    app.move_back_word();
                } else {
                    app.on_left();
                }
            }
            KeyCode::Up if event.modifiers.contains(KeyModifiers::ALT) => app.on_pgup(),
            KeyCode::Up => app.select_previous_channel(),
            KeyCode::Right => {
                if event
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
                {
                    app.move_forward_word();
                } else {
                    app.on_right();
                }
            }
            KeyCode::Down if event.modifiers.contains(KeyModifiers::ALT) => app.on_pgdn(),
            KeyCode::Down => app.select_next_channel(),
            KeyCode::PageUp => app.on_pgup(),
            KeyCode::PageDown => app.on_pgdn(),
            KeyCode::Char('f') if event.modifiers.contains(KeyModifiers::ALT) => {
                app.move_forward_word();
            }
            KeyCode::Char('b') if event.modifiers.contains(KeyModifiers::ALT) => {
                app.move_back_word();
            }
            KeyCode::Char('a') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                app.on_home();
            }
            KeyCode::Char('e') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                app.on_end();
            }
            KeyCode::Char('w') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                app.on_delete_word();
            }
            KeyCode::Char('j') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                app.select_next_channel();
            }
            KeyCode::Char('k') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                app.select_previous_channel();
            }
            KeyCode::Backspace
                if event
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                app.on_delete_word();
            }
            KeyCode::Char('k') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                app.on_delete_suffix();
            }
            code => app.on_key(code).await,
        },
        Event::Message(content) => {
            if let Err(e) = app.on_message(content).await {
                error!("failed on incoming message: {}", e);
            }
        }
        Event::Resize { .. } | Event::Redraw => {
            // will just redraw the app
        }
        Event::Quit(Some(e)) => return Err(e),
        Event::Quit(None) => {
            return Ok(None);
        }
    }

    Ok(Some(app))
}
