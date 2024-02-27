use gpui::*;

use super::piece_table::PieceTable;

pub struct State {
    pub text: PieceTable,
}

#[derive(Clone)]
pub struct StateModel {
    pub inner: Model<State>,
}

#[derive(Debug)]
pub enum TextEvent {
    Input { 
        keystroke: Keystroke,
        position: usize,
    },
    Movement(TextMovement),
    Submit {},
}

#[derive(Debug)]
pub enum TextMovement {
    Up,
    Down,
}

impl EventEmitter<TextEvent> for State {}

impl StateModel {
    pub fn init(cx: &mut WindowContext) {
        let model = cx.new_model(|_| State {
            text: PieceTable::new(""),
        });

        cx.subscribe(&model, |model, event: &TextEvent, cx| {
            std::dbg!("Incoming event: {}", event);
            let _ = cx.update_model(&model, |state, cx| {
                std::dbg!("Updating the model!");
                match event {
                    TextEvent::Input { keystroke, position } => {
                        match keystroke.key.as_str() {
                            "backspace" => {
                                if *position > 0 {
                                    state.text.delete(*position - 1, 1);
                                }
                            }
                            "enter" => {
                                state.text.insert(*position, "\n");
                            }
                            "space" => {
                                state.text.insert(*position, " ");
                            }
                            text => {
                                state.text.insert(*position, text);
                            }
                        }
                        
                    }
                    _ => {}
                }
                std::dbg!(state.text.content());
                cx.notify();
            });
        })
        .detach();

        let this = Self {
            inner: model,
        };

        cx.set_global::<StateModel>(this);
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut WindowContext), cx: &mut WindowContext) {
        cx.update_global::<Self, _>(|mut this, cx| {
            f(&mut this, cx);
        });
    }
}

impl Global for StateModel {}