//! Offline mode's Settings section (`backend::Settings::offline_mode`): its one option lets
//! offline names play without an account that owns the game, from the window, the command line
//! and the MCP server. It is on by default and shown with the rest of Settings; the seven clicks
//! on the sidebar's launcher version still open Settings here. The game still comes from the
//! player's own install: the launcher downloads Mojang's files only for an owner
//! (`backend::download_for_owner`).
use super::*;

/// Clicks on the launcher's version that reveal the section.
const CLICKS: u8 = 7;

impl Launcher {
    /// A click on the sidebar's launcher version; the last one opens Settings on the section.
    pub(super) fn version_clicked(&mut self) {
        if self.dev_options {
            return;
        }
        self.dev_clicks += 1;
        if self.dev_clicks >= CLICKS {
            self.dev_options = true;
            self.go(Page::Settings);
        }
    }

    /// The section: offline mode is on by default, so it shows with the rest of Settings.
    pub(super) fn dev_options_section(&self, cx: &mut Context<Self>) -> Option<Div> {
        let t = self.theme;
        let toggle = switch("dev-offline", self.settings.offline_mode(), &t).on_click(cx.listener(
            |this, _: &ClickEvent, _, cx| {
                this.settings.offline_mode = Some(!this.settings.offline_mode());
                this.save_settings();
                cx.notify();
            },
        ));
        Some(
            div()
                .flex()
                .flex_col()
                .child(
                    section_title("Developer options", "terminal", &t)
                        .mt(px(20.))
                        .mb(px(12.)),
                )
                .child(index_row(
                    0,
                    tr("Offline names without an account"),
                    Some(
                        div()
                            .child(tr(
                                "Offline names play without a Microsoft account that owns the game. \
                                 Minecraft still comes from your own install.",
                            ))
                            .into_any_element(),
                    ),
                    Some(toggle.into_any_element()),
                    &t,
                )),
        )
    }
}
