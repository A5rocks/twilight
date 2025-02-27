use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Type of interaction.
///
/// See [Discord Docs/Interaction Object].
///
/// [Discord Docs/Interaction Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-type
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    /// Interaction involves a message [`Component`].
    ///
    /// [`Component`]: super::super::component::Component
    MessageComponent = 3,
    /// Interaction involves a autocomplete request.
    ApplicationCommandAutocomplete = 4,
}

impl InteractionType {
    pub const fn kind(self) -> &'static str {
        match self {
            Self::Ping => "Ping",
            Self::ApplicationCommand => "ApplicationCommand",
            Self::MessageComponent => "MessageComponent",
            Self::ApplicationCommandAutocomplete => "ApplicationCommandAutocomplete",
        }
    }
}

#[derive(Debug)]
pub struct UnknownInteractionTypeError {
    value: u8,
}

impl Display for UnknownInteractionTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("unknown interaction type: ")?;

        Display::fmt(&self.value, f)
    }
}

impl TryFrom<u8> for InteractionType {
    type Error = UnknownInteractionTypeError;

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(Self::Ping),
            2 => Ok(Self::ApplicationCommand),
            3 => Ok(Self::MessageComponent),
            4 => Ok(Self::ApplicationCommandAutocomplete),
            other => Err(UnknownInteractionTypeError { value: other }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InteractionType, UnknownInteractionTypeError};
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        InteractionType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        Send,
        Sync
    );
    const_assert_eq!(1, InteractionType::Ping as u8);
    const_assert_eq!(2, InteractionType::ApplicationCommand as u8);
    const_assert_eq!(3, InteractionType::MessageComponent as u8);
    const_assert_eq!(4, InteractionType::ApplicationCommandAutocomplete as u8);

    #[test]
    fn test_kind() {
        assert_eq!("Ping", InteractionType::Ping.kind());
        assert_eq!(
            "ApplicationCommand",
            InteractionType::ApplicationCommand.kind()
        );
        assert_eq!("MessageComponent", InteractionType::MessageComponent.kind());
        assert_eq!(
            "ApplicationCommandAutocomplete",
            InteractionType::ApplicationCommandAutocomplete.kind()
        );
    }

    #[test]
    fn test_try_from() -> Result<(), UnknownInteractionTypeError> {
        assert_eq!(InteractionType::Ping, InteractionType::try_from(1)?);
        assert_eq!(
            InteractionType::ApplicationCommand,
            InteractionType::try_from(2)?
        );
        assert_eq!(
            InteractionType::MessageComponent,
            InteractionType::try_from(3)?
        );
        assert_eq!(
            InteractionType::ApplicationCommandAutocomplete,
            InteractionType::try_from(4)?,
        );
        assert!(InteractionType::try_from(u8::MAX).is_err());

        Ok(())
    }
}
