use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::AcceptType,
    protocol::{helpers::deserialize_skip_error, context::WithContext},
    traits::ActivityHandler,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    AppData,
    AppError,
    protocol::activities::Follow,
    entities::user::Model as DbUser,
    utilities::generate_activity_id
};

/// https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/protocol/activities/following/accept.rs
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptFollow {
    pub(crate) actor: ObjectId<DbUser>,
    /// Optional, for compatibility with platforms that always expect recipient field
    #[serde(deserialize_with = "deserialize_skip_error", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) to: Option<[ObjectId<DbUser>; 1]>,
    pub(crate) object: Follow,
    #[serde(rename = "type")]
    pub(crate) kind: AcceptType,
    pub(crate) id: Url,
}

/// https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/activities/following/accept.rs
/// https://github.com/LemmyNet/activitypub-federation-rust/blob/7bb17f21d59b0aed6126d8a8a0cd60897cb02e6d/examples/local_federation/activities/accept.rs
impl AcceptFollow {
    pub async fn new(follow: Follow, data: &Data<AppData>) -> Result<WithContext<Self>, AppError> {
        // 被关注者（本地账号），https://{}/u/{}
        let user: DbUser = follow.object.dereference_local(data).await?;
        // 关注者
        let person = follow.actor.clone().dereference(data).await?;
        // 接受关注
        let accept = AcceptFollow {
            actor: Url::parse(&user.id)?.into(),
            to: Some([Url::parse(&person.id)?.into()]),
            object: follow,
            kind: AcceptType::Accept,
            // 使用 UUID v7 作为 ID
            id: generate_activity_id(data.domain(), None)?
        };

        Ok(WithContext::new_default(accept))
    }
}

/// 只发送，不接收
/// send only, without receive
/// https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/activities/following/accept.rs
#[async_trait::async_trait]
impl ActivityHandler for AcceptFollow {
    type DataType = AppData;
    type Error = AppError;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        // TODO
        Ok(())
    }

    async fn receive(self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        // TODO
        Ok(())
    }
}
