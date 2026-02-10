use crossterm::event::{Event, EventStream};
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::types::AppEvent;

/// Spawn a task that reads terminal events and forwards them.
pub fn spawn_event_reader(tx: mpsc::UnboundedSender<AppEvent>, cancel: CancellationToken) {
    tokio::spawn(async move {
        let mut reader = EventStream::new();
        loop {
            tokio::select! {
                _ = cancel.cancelled() => break,
                event = reader.next() => {
                    match event {
                        Some(Ok(Event::Key(key))) => {
                            if tx.send(AppEvent::Key(key)).is_err() {
                                break;
                            }
                        }
                        Some(Ok(Event::Mouse(mouse))) => {
                            if tx.send(AppEvent::Mouse(mouse)).is_err() {
                                break;
                            }
                        }
                        Some(Ok(Event::Resize(w, h))) => {
                            if tx.send(AppEvent::Resize(w, h)).is_err() {
                                break;
                            }
                        }
                        Some(Err(_)) | None => break,
                        _ => {}
                    }
                }
            }
        }
    });
}
