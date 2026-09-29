use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, MouseButton, SharedString,
    Window, WindowBounds, WindowOptions,
};

use milagre::{ConnectionLifetime, ConnectionType, Coordinator, EventKind};

struct WorkspaceView {
    coordinator: Coordinator,
    left_worktree: u64,
    right_worktree: u64,
    selected_recipient: Option<u64>,
    show_raw: bool,
    drag_origin: Option<u64>,
    connection_kind: ConnectionType,
    connection_lifetime: ConnectionLifetime,
}

impl Render for WorkspaceView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let left = self
            .coordinator
            .worktrees
            .get(&self.left_worktree)
            .expect("left worktree");
        let right = self
            .coordinator
            .worktrees
            .get(&self.right_worktree)
            .expect("right worktree");
        let left_context = self.coordinator.context_for(self.left_worktree);
        let right_context = self.coordinator.context_for(self.right_worktree);
        let left_session = self
            .coordinator
            .sessions
            .values()
            .find(|session| session.worktree_id == self.left_worktree);
        let right_session = self
            .coordinator
            .sessions
            .values()
            .find(|session| session.worktree_id == self.right_worktree);
        let left_session_id = left_session.map(|session| session.id);
        let right_session_id = right_session.map(|session| session.id);
        let selected_name = self
            .selected_recipient
            .and_then(|id| self.coordinator.sessions.get(&id))
            .map(|session| session.agent_name.as_str())
            .unwrap_or("nenhum agente selecionado");
        let pending_approvals = self
            .coordinator
            .approvals
            .iter()
            .filter(|approval| approval.approved.is_none())
            .count();
        let attention_count = pending_approvals
            + self
                .coordinator
                .conflicts
                .iter()
                .filter(|conflict| conflict.blocked)
                .count();
        let connection_label = self
            .coordinator
            .connections
            .values()
            .next()
            .map(|connection| format_connection_type(&connection.kind))
            .unwrap_or("none".to_string());
        let raw_output = self
            .selected_recipient
            .and_then(|session_id| self.coordinator.latest_output_for(session_id))
            .map(|output| output.raw.clone())
            .unwrap_or_else(|| "No raw output captured yet.".to_string());

        let left_id_for_click = self.left_worktree;
        let right_id_for_click = self.right_worktree;
        let right_id_for_left_drop = self.right_worktree;
        let left_id_for_right_drop = self.left_worktree;
        let left_session_id_for_click = left_session_id;
        let right_session_id_for_click = right_session_id;
        let raw_button_label = if self.show_raw {
            "Hide raw output"
        } else {
            "Show raw output"
        };
        let selected_left = self.selected_recipient == left_session_id;
        let selected_right = self.selected_recipient == right_session_id;
        let connection_action_label = format!(
            "Link as {} · {}",
            format_connection_type(&self.connection_kind),
            format_connection_lifetime(&self.connection_lifetime)
        );

        div()
            .size_full()
            .bg(rgb(0x0f1117))
            .text_color(rgb(0xe7eaf0))
            .p_6()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Milagre"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x8d96a8))
                            .child("Agent coordination workspace"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(status_pill("2 agents", 0x6ee7b7))
                    .child(status_pill(
                        &format!("{} connection", self.coordinator.connections.len()),
                        0x93c5fd,
                    ))
                    .child(status_pill("local-first", 0xfcd34d)),
            )
            .child(
                div()
                    .flex_1()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(0x252b38))
                    .bg(rgb(0x151923))
                    .p_6()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_4()
                    .child(
                        div()
                            .id(SharedString::from("worktree-left"))
                            .child(worktree_card(
                                &left.name,
                                left_session
                                    .map(|session| session.agent_name.as_str())
                                    .unwrap_or("no agent"),
                                left_context.events.len(),
                                selected_left,
                            ))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.selected_recipient = left_session_id_for_click;
                                cx.notify();
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.drag_origin = Some(left_id_for_click);
                                    cx.notify();
                                }),
                            )
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    if let Some(origin) = this.drag_origin.take() {
                                        if origin != right_id_for_left_drop {
                                            this.coordinator.connect(
                                                origin,
                                                right_id_for_left_drop,
                                                this.connection_kind.clone(),
                                                this.connection_lifetime.clone(),
                                            );
                                        }
                                    }
                                    cx.notify();
                                }),
                            ),
                    )
                    .child(connection_marker(&connection_label))
                    .child(
                        div()
                            .id(SharedString::from("worktree-right"))
                            .child(worktree_card(
                                &right.name,
                                right_session
                                    .map(|session| session.agent_name.as_str())
                                    .unwrap_or("no agent"),
                                right_context.events.len(),
                                selected_right,
                            ))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.selected_recipient = right_session_id_for_click;
                                cx.notify();
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.drag_origin = Some(right_id_for_click);
                                    cx.notify();
                                }),
                            )
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    if let Some(origin) = this.drag_origin.take() {
                                        if origin != left_id_for_right_drop {
                                            this.coordinator.connect(
                                                origin,
                                                left_id_for_right_drop,
                                                this.connection_kind.clone(),
                                                this.connection_lifetime.clone(),
                                            );
                                        }
                                    }
                                    cx.notify();
                                }),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(info_panel(
                        "Shared context",
                        if left_context.summary.is_empty() {
                            "No shared events yet."
                        } else {
                            &left_context.summary
                        },
                    ))
                    .child(info_panel(
                        "Attention",
                        &format!(
                            "{} item(s) need your attention: {} approval(s), {} blocked conflict(s).",
                            attention_count,
                            pending_approvals,
                            self.coordinator
                                .conflicts
                                .iter()
                                .filter(|conflict| conflict.blocked)
                                .count()
                        ),
                    )),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .flex_1()
                            .rounded_lg()
                            .bg(rgb(0x151923))
                            .border_1()
                            .border_color(rgb(0x252b38))
                            .p_4()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("Global chat"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x9aa5b8))
                                    .child(format!("Recipient: {}", selected_name)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        action_button("Select API agent")
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.selected_recipient = left_session_id;
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        action_button("Select web agent")
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.selected_recipient = right_session_id;
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        action_button("Send context ping")
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                if let Some(session_id) = this.selected_recipient {
                                                    let summary = this
                                                        .coordinator
                                                        .context_for(this.right_worktree)
                                                        .summary;
                                                    this.coordinator.send_message(
                                                        session_id,
                                                        if summary.is_empty() {
                                                            "Please report your current status."
                                                        } else {
                                                            "Please use the latest connected context and report your next action."
                                                        },
                                                    );
                                                }
                                                cx.notify();
                                            })))
                                    .child(
                                        action_button(connection_action_label).on_click(
                                            cx.listener(|this, _, _, cx| {
                                                this.connection_kind = next_connection_type(
                                                    &this.connection_kind,
                                                );
                                                cx.notify();
                                            }),
                                        ),
                                    )
                                    .child(
                                        action_button("Toggle temporary link").on_click(
                                            cx.listener(|this, _, _, cx| {
                                                this.connection_lifetime =
                                                    match this.connection_lifetime {
                                                        ConnectionLifetime::Persistent => {
                                                            ConnectionLifetime::Temporary
                                                        }
                                                        ConnectionLifetime::Temporary => {
                                                            ConnectionLifetime::Persistent
                                                        }
                                                    };
                                                cx.notify();
                                            }),
                                        ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .rounded_lg()
                            .bg(rgb(0x151923))
                            .border_1()
                            .border_color(rgb(0x252b38))
                            .p_4()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child("Evidence"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x9aa5b8))
                                    .child(if self.show_raw {
                                        raw_output
                                    } else {
                                        "Summaries stay short. Expand to inspect raw agent output."
                                            .to_string()
                                    }),
                            )
                            .child(
                                action_button(raw_button_label).on_click(cx.listener(
                                    |this, _, _, cx| {
                                        this.show_raw = !this.show_raw;
                                        cx.notify();
                                    },
                                )),
                            ),
                    ),
            )
    }
}

fn status_pill(label: &str, color: u32) -> impl IntoElement {
    div()
        .rounded_full()
        .bg(rgb(0x202735))
        .px_3()
        .py_1()
        .text_xs()
        .child(div().text_color(rgb(color)).child("● "))
        .child(label.to_string())
}

fn worktree_card(
    name: &str,
    agent: &str,
    context_count: usize,
    selected: bool,
) -> impl IntoElement {
    div()
        .w(px(260.))
        .rounded_lg()
        .border_1()
        .border_color(rgb(if selected { 0x6ee7b7 } else { 0x34405a }))
        .bg(rgb(0x1b2130))
        .p_4()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_lg()
                .font_weight(gpui::FontWeight::BOLD)
                .child(name.to_string()),
        )
        .child(
            div()
                .text_sm()
                .text_color(rgb(0x9aa5b8))
                .child(format!("{} · running", agent)),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(0x6ee7b7))
                .child(format!("{} shared events", context_count)),
        )
}

fn connection_marker(kind: &str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .gap_2()
        .child(div().text_color(rgb(0x93c5fd)).child("↔"))
        .child(
            div()
                .text_xs()
                .text_color(rgb(0x77829a))
                .child(kind.to_string()),
        )
}

fn action_button(label: impl Into<String>) -> gpui::Stateful<gpui::Div> {
    let label = label.into();
    div()
        .id(SharedString::from(label.clone()))
        .rounded_sm()
        .bg(rgb(0x202735))
        .text_xs()
        .px_2()
        .py_1()
        .cursor_pointer()
        .active(|this| this.opacity(0.75))
        .child(label)
}

fn format_connection_type(kind: &ConnectionType) -> String {
    match kind {
        ConnectionType::Dependency => "dependency".to_string(),
        ConnectionType::Information => "information".to_string(),
        ConnectionType::Review => "review".to_string(),
        ConnectionType::Blocking => "blocking".to_string(),
    }
}

fn format_connection_lifetime(lifetime: &ConnectionLifetime) -> &'static str {
    match lifetime {
        ConnectionLifetime::Persistent => "persistent",
        ConnectionLifetime::Temporary => "temporary",
    }
}

fn next_connection_type(kind: &ConnectionType) -> ConnectionType {
    match kind {
        ConnectionType::Dependency => ConnectionType::Information,
        ConnectionType::Information => ConnectionType::Review,
        ConnectionType::Review => ConnectionType::Blocking,
        ConnectionType::Blocking => ConnectionType::Dependency,
    }
}

fn info_panel(title: &str, body: &str) -> impl IntoElement {
    div()
        .flex_1()
        .rounded_lg()
        .bg(rgb(0x151923))
        .border_1()
        .border_color(rgb(0x252b38))
        .p_4()
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::BOLD)
                .child(title.to_string()),
        )
        .child(
            div()
                .mt_1()
                .text_xs()
                .text_color(rgb(0x8d96a8))
                .child(body.to_string()),
        )
}

fn main() {
    let mut coordinator = Coordinator::new();
    let project = coordinator.add_project("Milagre");
    let left = coordinator.add_worktree(project, "api", "./worktrees/api");
    let right = coordinator.add_worktree(project, "web", "./worktrees/web");
    coordinator.start_session(left, "agent-a");
    coordinator.start_session(right, "agent-b");
    coordinator.connect(
        left,
        right,
        ConnectionType::Information,
        ConnectionLifetime::Persistent,
    );
    coordinator.publish(
        left,
        EventKind::Decision,
        "Shared API contract",
        "The web agent can consume the API contract.",
    );

    Application::new().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1040.), px(720.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| WorkspaceView {
                    coordinator,
                    left_worktree: left,
                    right_worktree: right,
                    selected_recipient: None,
                    show_raw: false,
                    drag_origin: None,
                    connection_kind: ConnectionType::Information,
                    connection_lifetime: ConnectionLifetime::Persistent,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
