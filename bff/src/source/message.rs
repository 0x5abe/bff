use crate::names::Name;
use crate::source::keyframer::SourceLinearTrack;

pub type SourceMessageTrack = SourceLinearTrack<Vec<SourceMessage>>;

pub enum SourceMessage {
    Command {
        command: String,
        args: Vec<String>,
    },
    Raw {
        message_class: u32,
        receiver_name: Name,
        unknown_u32: u32,
        parameter: f32,
        message_name: Name,
    },
}
