use crate::app::{Channel, ChannelId, Message};
use crate::util::StatefulList;

use uuid::Uuid;

use std::convert::TryInto;
use std::path::Path;

const CHANNELS_TREE: &str = "gurk-channels";
const MESSAGES_TREE: &str = "gurk-messages";
const NAMES_TREE: &str = "gurk-names";
const INPUT_TREE: &str = "gurk-input";
const INPUT_KEY: &str = "input";

pub struct Store {
    db: sled::Db,
    channels: sled::Tree,
    messages: sled::Tree,
    names: sled::Tree,
    input: sled::Tree,
}

impl Store {
    pub fn open(path: impl AsRef<Path>) -> sled::Result<Self> {
        let db = sled::open(path)?;
        let channels = db.open_tree(CHANNELS_TREE)?;
        let messages = db.open_tree(MESSAGES_TREE)?;
        let names = db.open_tree(NAMES_TREE)?;
        let input = db.open_tree(INPUT_TREE)?;
        Ok(Self {
            db,
            channels,
            messages,
            names,
            input,
        })
    }

    pub fn channels(&self) -> impl Iterator<Item = Channel> + '_ {
        self.channels.iter().filter_map(move |res| {
            let (_k, v) = res.ok()?;
            let mut channel: Channel = serde_json::from_slice(&v).ok()?;
            channel.messages =
                StatefulList::with_items(self.channel_messages(channel.id).collect());
            Some(channel)
        })
    }

    pub fn channel_messages(&self, channel_id: ChannelId) -> impl Iterator<Item = Message> {
        let k = channel_id.to_bytes();
        self.messages.scan_prefix(k).filter_map(|res| {
            let (_k, v) = res.ok()?;
            serde_json::from_slice(&v).ok()
        })
    }

    pub fn push_channel(&self, channel: &Channel) -> sled::Result<()> {
        let k = channel.id.to_bytes();
        let v = serde_json::to_vec(channel).map_err(from_json_error)?;
        self.channels.insert(k, v)?;
        Ok(())
    }

    pub fn push_message(&self, channel_id: ChannelId, message: &Message) -> sled::Result<()> {
        let message_id = self.db.generate_id()?;
        let k = MessageKey {
            channel_id,
            message_id,
        }
        .to_bytes();
        let v = serde_json::to_vec(message).map_err(from_json_error)?;
        self.messages.insert(k, v)?;
        Ok(())
    }

    pub fn names(&self) -> impl Iterator<Item = (Uuid, String)> {
        self.names.iter().filter_map(|res| {
            let (k, v) = res.ok()?;
            let k = k.as_ref().try_into().ok()?;
            let id = Uuid::from_bytes(k);
            let name = std::str::from_utf8(&v).ok()?.to_string();
            Some((id, name))
        })
    }

    pub fn push_name(&self, id: Uuid, name: &str) -> sled::Result<()> {
        self.names.insert(id.as_bytes(), name)?;
        Ok(())
    }

    pub fn input(&self) -> sled::Result<String> {
        Ok(self
            .input
            .get(INPUT_KEY)?
            .map(|v| String::from_utf8_lossy(&v).to_string())
            .unwrap_or_default())
    }
}

impl ChannelId {
    const BYTES_LEN: usize = 33;

    /// Result must preserve the natural ordering of `ChannelId`.
    fn to_bytes(&self) -> [u8; Self::BYTES_LEN] {
        let mut bytes = [0; Self::BYTES_LEN];
        match self {
            ChannelId::User(uuid) => {
                bytes[0] = 0;
                bytes[1..17].copy_from_slice(uuid.as_bytes());
            }
            ChannelId::Group(master_key) => {
                bytes[0] = 1;
                bytes[1..33].copy_from_slice(master_key);
            }
        }
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            [0, data @ ..] if data.len() + 1 == Self::BYTES_LEN => {
                let uuid = Uuid::from_bytes(data[..16].try_into().ok()?);
                Some(ChannelId::User(uuid))
            }
            [1, data @ ..] if data.len() + 1 == Self::BYTES_LEN => {
                let master_key = data.try_into().ok()?;
                Some(ChannelId::Group(master_key))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MessageKey {
    channel_id: ChannelId,
    message_id: u64,
}

impl MessageKey {
    const BYTES_LEN: usize = ChannelId::BYTES_LEN + 8;

    /// Result must preserve the natural ordering of `MessageKey`.
    fn to_bytes(&self) -> [u8; Self::BYTES_LEN] {
        let mut bytes = [0; Self::BYTES_LEN];
        bytes[..ChannelId::BYTES_LEN].copy_from_slice(&self.channel_id.to_bytes());
        bytes[ChannelId::BYTES_LEN..].copy_from_slice(&self.message_id.to_be_bytes());
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let channel_id = ChannelId::from_bytes(&bytes[..ChannelId::BYTES_LEN])?;
        let message_id = u64::from_be_bytes(bytes[ChannelId::BYTES_LEN..].try_into().ok()?);
        Some(Self {
            channel_id,
            message_id,
        })
    }
}

fn from_json_error(e: serde_json::Error) -> sled::Error {
    sled::Error::Unsupported(format!("failed to convert from json: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for ChannelId {
        fn arbitrary(g: &mut Gen) -> Self {
            if bool::arbitrary(g) {
                let uuid = Uuid::from_bytes(u128::arbitrary(g).to_be_bytes());
                Self::User(uuid)
            } else {
                let mut master_key = [0; 32];
                master_key[..16].copy_from_slice(&u128::arbitrary(g).to_be_bytes());
                master_key[16..].copy_from_slice(&u128::arbitrary(g).to_be_bytes());
                Self::Group(master_key)
            }
        }
    }

    impl Arbitrary for MessageKey {
        fn arbitrary(g: &mut Gen) -> Self {
            Self {
                channel_id: ChannelId::arbitrary(g),
                message_id: u64::arbitrary(g),
            }
        }
    }

    #[quickcheck]
    fn test_channel_id_bytes(channel_id: ChannelId) -> bool {
        ChannelId::from_bytes(&channel_id.to_bytes()) == Some(channel_id)
    }

    #[quickcheck]
    fn test_channel_id_bytes_cmp(a: ChannelId, b: ChannelId) -> bool {
        a.cmp(&b) == a.to_bytes().cmp(&b.to_bytes())
    }

    #[quickcheck]
    fn test_message_key_bytes(message_key: MessageKey) -> bool {
        MessageKey::from_bytes(&message_key.to_bytes()) == Some(message_key)
    }

    #[quickcheck]
    fn test_message_key_bytes_cmp(a: MessageKey, b: MessageKey) -> bool {
        a.cmp(&b) == a.to_bytes().cmp(&b.to_bytes())
    }
}
