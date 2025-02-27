use crate::{
    client::Client,
    error::Error,
    request::{self, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    invite::{WelcomeScreen, WelcomeScreenChannel},
};

#[derive(Serialize)]
struct UpdateGuildWelcomeScreenFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "request::slice_is_empty")]
    welcome_channels: &'a [WelcomeScreenChannel],
}

/// Update the guild's welcome screen.
///
/// Requires the [`MANAGE_GUILD`] permission.
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildWelcomeScreen<'a> {
    fields: UpdateGuildWelcomeScreenFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> UpdateGuildWelcomeScreen<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: UpdateGuildWelcomeScreenFields {
                description: None,
                enabled: None,
                welcome_channels: &[],
            },
            guild_id,
            http,
        }
    }

    /// Set the description of the welcome screen.
    pub const fn description(mut self, description: &'a str) -> Self {
        self.fields.description = Some(description);

        self
    }

    /// Set whether the welcome screen is enabled.
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.fields.enabled = Some(enabled);

        self
    }

    /// Set the channels linked in the welcome screen, with associated metadata.
    pub const fn welcome_channels(mut self, welcome_channels: &'a [WelcomeScreenChannel]) -> Self {
        self.fields.welcome_channels = welcome_channels;

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<WelcomeScreen> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateGuildWelcomeScreen<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateGuildWelcomeScreen {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
