use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Window, WindowBounds,
    WindowOptions,
};

use milagre::{ConnectionLifetime, ConnectionType, Coordinator, EventKind};

struct WorkspaceView {
    coordinator: Coordinator,
    left_worktree: u64,
    right_worktree: u64,
}

impl Render for WorkspaceView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child(status_pill("1 connection", 0x93c5fd))
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
                    .child(worktree_card(
                        &left.name,
                        "agent-a",
                        left_context.events.len(),
                    ))
                    .child(connection_marker())
                    .child(worktree_card(
                        &right.name,
                        "agent-b",
                        right_context.events.len(),
                    )),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(info_panel(
                        "Shared context",
                        "Decisions and blockers flow across connected worktrees.",
                    ))
                    .child(info_panel(
                        "Attention",
                        "No approvals or conflicts require action.",
                    )),
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

fn worktree_card(name: &str, agent: &str, context_count: usize) -> impl IntoElement {
    div()
        .w(px(260.))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0x34405a))
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

fn connection_marker() -> impl IntoElement {
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
                .child("information"),
        )
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
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
