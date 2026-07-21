pub mod explorer;
pub mod inspector;
pub mod viewport;
pub mod sequencer;
pub mod event_forge;
pub mod audio;

pub use explorer::ExplorerDockable;
pub use inspector::InspectorDockable;
pub use viewport::ViewportDockable;
pub use sequencer::SequencerDockable;
pub use event_forge::EventForgeDockable;
pub use audio::AudioDockable;

pub fn create_dockables() -> (
    ExplorerDockable,
    InspectorDockable,
    ViewportDockable,
    SequencerDockable,
    EventForgeDockable,
    AudioDockable,
) {
    (
        ExplorerDockable::default(),
        InspectorDockable::default(),
        ViewportDockable::default(),
        SequencerDockable::default(),
        EventForgeDockable::default(),
        AudioDockable::default(),
    )
}

