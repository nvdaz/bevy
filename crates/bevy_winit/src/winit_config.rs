use bevy_ecs::system::Resource;
use bevy_utils::Duration;

/// Settings for the [`WinitPlugin`](super::WinitPlugin).
#[derive(Debug, Resource)]
pub struct WinitSettings {
    /// Determines how frequently the application can update when it has focus.
    pub focused_mode: UpdateMode,
    /// Determines how frequently the application can update when it's out of focus.
    pub unfocused_mode: UpdateMode,
}

impl WinitSettings {
    /// Default settings for games.
    ///
    /// [`Continuous`](UpdateMode::Continuous) if windows have focus,
    /// [`ReactiveLowPower`](UpdateMode::ReactiveLowPower) otherwise.
    pub fn game() -> Self {
        WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::ReactiveLowPower {
                wait: Duration::from_secs_f64(1.0 / 60.0), // 60Hz
            },
        }
    }

    /// Default settings for desktop applications.
    ///
    /// [`Reactive`](UpdateMode::Reactive) if windows have focus,
    /// [`ReactiveLowPower`](UpdateMode::ReactiveLowPower) otherwise.
    pub fn desktop_app() -> Self {
        WinitSettings {
            focused_mode: UpdateMode::Reactive {
                wait: Duration::from_secs(5),
            },
            unfocused_mode: UpdateMode::ReactiveLowPower {
                wait: Duration::from_secs(60),
            },
        }
    }

    /// Returns the current [`UpdateMode`].
    ///
    /// **Note:** The output depends on whether the window has focus or not.
    pub fn update_mode(&self, focused: bool) -> &UpdateMode {
        match focused {
            true => &self.focused_mode,
            false => &self.unfocused_mode,
        }
    }
}

impl Default for WinitSettings {
    fn default() -> Self {
        WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        }
    }
}

/// Determines how frequently an [`App`](bevy_app::App) should update.
///
/// **Note:** This setting is independent of VSync. VSync is controlled by a window's
/// [`PresentMode`](bevy_window::PresentMode) setting. If an app can update faster than the refresh
/// rate, but VSync is enabled, the update rate will be indirectly limited by the renderer.
#[derive(Debug, Clone, Copy)]
pub enum UpdateMode {
    /// The [`App`](bevy_app::App) will update over and over, as fast as it possibly can, until an
    /// [`AppExit`](bevy_app::AppExit) event appears.
    Continuous,
    /// The [`App`](bevy_app::App) will update in response to the following, until an
    /// [`AppExit`](bevy_app::AppExit) event appears:
    /// - `wait` time has elapsed since the previous update
    /// - a redraw has been requested by [`RequestRedraw`](bevy_window::RequestRedraw)
    /// - new [window](`winit::event::WindowEvent`) or [raw input](`winit::event::DeviceEvent`)
    /// events have appeared
    Reactive {
        /// The minimum time from the start of one update to the next.
        ///
        /// **Note:** This has no upper limit.
        /// The [`App`](bevy_app::App) will wait indefinitely if you set this to [`Duration::MAX`].
        wait: Duration,
    },
    /// The [`App`](bevy_app::App) will update in response to the following, until an
    /// [`AppExit`](bevy_app::AppExit) event appears:
    /// - `wait` time has elapsed since the previous update
    /// - a redraw has been requested by [`RequestRedraw`](bevy_window::RequestRedraw)
    /// - new [window events](`winit::event::WindowEvent`) have appeared
    ///
    /// **Note:** Unlike [`Reactive`](`UpdateMode::Reactive`), this mode will ignore events that
    /// don't come from interacting with a window, like [`MouseMotion`](winit::event::DeviceEvent::MouseMotion).
    /// Use this mode if, for example, you only want your app to update when the mouse cursor is
    /// moving over a window, not just moving in general. This can greatly reduce power consumption.
    ReactiveLowPower {
        /// The minimum time from the start of one update to the next.
        ///
        /// **Note:** This has no upper limit.
        /// The [`App`](bevy_app::App) will wait indefinitely if you set this to [`Duration::MAX`].
        wait: Duration,
    },
}
