pub const OVK_LEN_BYTES: usize = 32;

/// Allows viewing outgoing notes, i.e., notes sent from the spending key this
/// key is derived from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutgoingViewingKey(pub(crate) [u8; OVK_LEN_BYTES]);

impl OutgoingViewingKey {
    pub fn to_bytes(&self) -> [u8; OVK_LEN_BYTES] {
        self.0
    }
}
